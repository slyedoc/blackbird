use bevy::{
    asset::{
        AssetPlugin,
        processor::{AssetProcessor, ProcessorState},
    },
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future},
};

/// This generates imported_assets for CI/CD
// TODO: does bevy already have this?
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Processed,
            ..default()
        }))
        .add_systems(Update, check_asset_processor)
        .run();
}

#[derive(Component)]
struct AssetProcessingTask(Task<ProcessorState>);

// dont know of away way to check the status without async, so we just spawn task to check
fn check_asset_processor(
    mut commands: Commands,
    asset_processsor: Res<AssetProcessor>,
    mut tasks: Query<(Entity, &mut AssetProcessingTask)>,
) {
    let mut found = false;
    // check for existing tasks
    for (e, mut task) in tasks.iter_mut() {
        found = true;
        // check the status, if complete, remove the task
        if let Some(status) = block_on(future::poll_once(&mut task.0)) {
            info!("Asset processer state: {:?}", state_name(status));
            commands.entity(e).despawn();
            found = false;

            if status == ProcessorState::Finished {
                commands.send_event(AppExit::Success);
            }
        }
    }

    // if we didn't find a task or its done, spawn a new one
    if !found {
        let thread_pool = AsyncComputeTaskPool::get();
        let proc = asset_processsor.clone();
        let task = thread_pool.spawn(async move {
            let status = proc.get_state().await;
            status
        });

        // Spawn new entity and add our new task as a component
        commands.spawn(AssetProcessingTask(task));
    }
}

fn state_name(status: ProcessorState) -> &'static str {
    match status {
        ProcessorState::Initializing => "Initializing",
        ProcessorState::Processing => "Processing",
        ProcessorState::Finished => "Finished",
    }
}
