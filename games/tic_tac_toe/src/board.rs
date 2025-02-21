use bevy::prelude::*;
use avian3d::prelude::*;

use bevy_inspector_egui::prelude::*;
use rand::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TicTacToeAssets>()
        .add_systems(Update, asset_changed.run_if(resource_changed::<TicTacToeAssets>))
            .add_observer(on_add_board);
    }
}

fn asset_changed(
    mut commands: Commands,
    query: Query<(Entity, &Children), With<TicTacToeBoard>>,
) {
    for (e, children) in query.iter() {
        for c in children.iter() {
            if let Some(mut cmd) = commands.get_entity(*c) {
                cmd.despawn();
            }
        }

        commands.entity(e)
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .remove::<RigidBody>()
            .remove::<Collider>();

        commands.trigger_targets(OnAdd, e);
    }
}

pub fn on_add_board(
    trigger: Trigger<OnAdd, TicTacToeBoard>,
    mut cmd: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    a: Res<TicTacToeAssets>,
) {
    let e = trigger.entity();    

    cmd.entity(e).insert((
        Mesh3d(a.board_mesh.clone()),
        MeshMaterial3d(a.board_mat.clone()),
        RigidBody::Static,
        Collider::cuboid(a.board_size, a.board_depth, a.board_size),
    ));

    // lines
    for i in 0..2 {                    
        let offset = -(a.board_size * 0.5) + ((a.board_size / 3.0) * i as f32 + a.board_size / 3.0);
        // Horizontal
        cmd.spawn((
            Transform {
                translation: Vec3::new(0.0, (a.board_depth * 0.5) + a.line_depth, offset),
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2) ,
                  
                ..Default::default()
            },
            Mesh3d(a.line_mesh.clone()),
            MeshMaterial3d(a.line_mat.clone()),
        ))
        .set_parent(e);

        //Vertical
        cmd.spawn((
            Transform {
                translation: Vec3::new(offset, (a.board_depth * 0.5) + a.line_depth, 0.0 ),
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2) 
                    * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..default()
            },
            Mesh3d(a.line_mesh.clone()),
            MeshMaterial3d(a.line_mat.clone()),
        ))
        .set_parent(e);
    }

    // spawn cells
    for x in 0..3 {
        let x_offset =
            -(a.board_size * 0.5) + ((a.board_size / 3.0) * x as f32 + a.board_size / 6.0);
        for y in 0..3 {
            let y_offset =
                -(a.board_size * 0.5) + ((a.board_size / 3.0) * y as f32 + a.board_size / 6.0);
            cmd.spawn((
                Transform {
                    translation: Vec3::new(x_offset, a.board_depth * 0.5, y_offset),
                    ..Default::default()
                },
                Mesh3d(a.cell_mesh.clone()),
                Collider::sphere(a.cell_size),
                Cell::new(x as u8, y as u8),
            ))
            .observe(on_click_cell)
            .observe(on_over_cell)
            .observe(on_out_cell)
            .set_parent(e);
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Cell {
    peice: Option<Player>,
    model: Option<Entity>,
    pos: u8,
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            peice: None,
            pos: y * 3 + x,
            model: None,
        }
    }
}

fn on_click_cell(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut cells: Query<(&mut Cell, &Parent)>,
    mut boards: Query<&mut TicTacToeBoard>,
    board_assets: Res<TicTacToeAssets>,
) {
    info!("on_click_cell");
    let e = trigger.entity();
    let (mut cell, parent) = cells.get_mut(trigger.entity()).unwrap();
    let mut board = boards.get_mut(parent.get()).unwrap();
    match cell.peice {
        Some(_) => (),
        None => {
            cell.peice = Some(board.player.clone());

            // delete preview
            if let Some(e) = cell.model {
                commands.entity(e).despawn();
                cell.model = None;
            }

            // update model
            commands
                .entity(e)
                .with_children(|parent| 
                    cell.model = match board.player {
                    Player::X => Some(spawn_x(&board_assets, parent, false)),
                    Player::O => Some(spawn_o(&board_assets, parent, false)),
                });

            board.make_move(cell.pos as usize);
        }
    }
}

fn spawn_x(board_assets: &Res<'_, TicTacToeAssets>, parent: &mut ChildBuilder<'_>, hover: bool) -> Entity {
    parent.spawn((
        Transform::default(),
        GlobalTransform::default(),
        InheritedVisibility::default()
    ))
    .with_children(|p| {
        p.spawn((
            Transform {
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                ..Default::default()
            },
            Mesh3d(board_assets.x_mesh.clone()),
            MeshMaterial3d( if !hover { board_assets.x_mat.clone() } else { board_assets.x_hover_mat.clone() }),
        ));
        p.spawn((
            Transform {
                rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4),
                ..default()
            },
            Mesh3d(board_assets.x_mesh.clone()),
            MeshMaterial3d( if !hover { board_assets.x_mat.clone() } else { board_assets.x_hover_mat.clone() }),
        ));
    }).id()
}

fn spawn_o(board_assets: &Res<'_, TicTacToeAssets>, parent: &mut ChildBuilder<'_>, hover: bool) -> Entity {
    parent.spawn((
        Transform {
            ..default()
        },
        Mesh3d(board_assets.o_mesh.clone()),
        MeshMaterial3d( if !hover { board_assets.o_mat.clone() } else { board_assets.o_hover_mat.clone() }),
    )).id()
}

fn on_over_cell(
    trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut cells: Query<(&mut Cell, &Parent)>,
    boards: Query<&TicTacToeBoard>,
    a: Res<TicTacToeAssets>,
) {
    info!("on_over_cell");
    let (mut cell, parent) = cells.get_mut(trigger.entity()).unwrap();
    let board = boards.get(parent.get()).unwrap();
    if cell.peice.is_none() && cell.model.is_none() {
        commands
            .entity(trigger.entity())
            .with_children(|parent| {
                let m = match board.player {
                    Player::X => spawn_x(&a, parent, true),
                    Player::O => spawn_o(&a, parent, true),
                };
                cell.model = Some(m);
            });
    }
}

fn on_out_cell(
    trigger: Trigger<Pointer<Out>>,
    mut commands: Commands,
    mut cells: Query<&mut Cell>,    
) {
    let mut cell = cells.get_mut(trigger.entity()).unwrap();
    if cell.peice.is_some() {
        return;
    }
    if let Some(m) = cell.model {        
        commands.entity(m).despawn_recursive();
        cell.model = None;
    }
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub enum Player {
    #[default]
    X,
    O,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct TicTacToeBoard {
    // The board is a 9-element array.
    // Each cell is either empty (None) or occupied by Some(Player).
    cells: [Option<Player>; 9],
    /// Current Player to make a move
    pub player: Player,
}

impl TicTacToeBoard {
    /// Attempts to place the player's marker at the given index (0–8).
    /// Returns true if the move was legal.
    pub fn make_move(&mut self, index: usize) -> bool {
        if index < 9 && self.cells[index].is_none() {
            self.cells[index] = Some(self.player);
            self.player = self.player.opponent();
            true
        } else {
            false
        }
    }

    /// Returns a vector of indices (0–8) for cells that are empty.
    pub fn get_legal_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_none() { Some(i) } else { None })
            .collect()
    }

    /// Checks all win lines and returns the winning player if there is one.
    pub fn check_winner(&self) -> Option<Player> {
        for line in LINES.iter() {
            if let (Some(a), Some(b), Some(c)) = (
                self.cells[line[0]],
                self.cells[line[1]],
                self.cells[line[2]],
            ) {
                if a == b && b == c {
                    return Some(a);
                }
            }
        }
        None
    }

    /// Returns true if the board is full and there is no winner.
    pub fn is_draw(&self) -> bool {
        self.get_legal_moves().is_empty() && self.check_winner().is_none()
    }

    /// Returns true if the game is over (win or draw).
    pub fn is_terminal(&self) -> bool {
        self.check_winner().is_some() || self.is_draw()
    }

    /// Prints the board.
    /// Empty cells are printed with their position number (1–9).
    pub fn print(&self) {
        for i in 0..9 {
            match self.cells[i] {
                Some(player) => print!(
                    " {} ",
                    match player {
                        Player::X => "X",
                        Player::O => "O",
                    }
                ),
                None => print!(" {} ", i + 1),
            }
            if i % 3 != 2 {
                print!("|");
            } else if i != 8 {
                println!("\n-----------");
            }
        }
        println!("\n");
    }
}

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TicTacToeAssets {
    board_size: f32,
    board_depth: f32,
    line_length: f32,
    line_width: f32,
    line_depth: f32,
    cell_size: f32,

    board_mat: Handle<StandardMaterial>,
    board_mesh: Handle<Mesh>,

    // caplet mesh, will use 2 of these
    x_mesh: Handle<Mesh>,
    x_mat: Handle<StandardMaterial>,
    x_hover_mat: Handle<StandardMaterial>,

    o_mesh: Handle<Mesh>,
    o_mat: Handle<StandardMaterial>,
    o_hover_mat: Handle<StandardMaterial>,

    cell_mesh: Handle<Mesh>,

    line_mat: Handle<StandardMaterial>,
    line_mesh: Handle<Mesh>,
}

impl FromWorld for TicTacToeAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let board_size = 1.0;
        let board_depth = 0.1;
        let line_length = board_size * 0.9;
        let line_width = 0.01;
        let line_depth = 0.025;
        let cell_size = 0.1;

        let board_mat = materials.add(Color::from(Color::srgb(0.5, 0.5, 0.5)));
        let line_mat = materials.add(Color::from(Color::srgb(0.8, 0.8, 0.8)));
        let x_mat = materials.add(Color::from(Color::srgb(1.0, 0.0, 0.0)));
        let x_hover_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0).with_alpha(0.5).into(),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        let o_mat = materials.add(Color::from(Color::srgb(0.0, 0.0, 1.0)));
        let o_hover_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0).with_alpha(0.5).into(),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let board_mesh = meshes.add(Cuboid::from_size(Vec3::new(
            board_size,
            board_depth,
            board_size,
        )));

        let x = line_length + line_width;
        let y = line_width;
        let z = line_depth;

        let line_mesh = meshes.add(Extrusion::new(Rectangle::new(x, y), z));

        let cell_mesh = meshes.add(Sphere::new(cell_size));
        let x_mesh = meshes.add(Capsule3d::new(0.03, 0.2));
        let o_mesh = meshes.add(Torus::new(0.05, 0.1));

        Self {
            board_size,
            board_depth,
            line_length,
            line_width,
            line_depth,
            cell_size,
            cell_mesh,

            board_mat,
            board_mesh,
            line_mat,
            line_mesh,
            x_mesh,
            x_mat,
            x_hover_mat,
            o_mesh,
            o_mat,
            o_hover_mat,
        }
    }
}

/// A node in the MCTS tree. Each node holds a game state, the move that led to
/// this state, and statistics for MCTS. Instead of using Rc/RefCell, we store the
/// tree in a Vec and refer to nodes by their indices.
pub struct TreeNode {
    board: TicTacToeBoard,
    /// The move (cell index) that was played to get here.
    move_played: Option<usize>,
    /// The indices of this node's children.
    children: Vec<usize>,
    wins: f64,
    visits: u32,
    /// Moves that have not yet been tried from this node.
    untried_moves: Vec<usize>,
}

impl TreeNode {
    fn new(board: TicTacToeBoard, move_played: Option<usize>) -> Self {
        let untried_moves = board.get_legal_moves();
        TreeNode {
            board,
            move_played,
            children: Vec::new(),
            wins: 0.0,
            visits: 0,
            untried_moves,
        }
    }

    /// Computes the UCT value for this node.
    /// If the node has not been visited, returns infinity.
    fn uct_value(&self, parent_visits: u32) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        (self.wins / self.visits as f64)
            + (2.0 * ((parent_visits as f64).ln() / self.visits as f64)).sqrt()
    }
}

/// Runs the MCTS algorithm starting from `root_board` and returns the best move (cell index)
/// for the given board
pub fn mcts_search(root_board: TicTacToeBoard, iterations: u32) -> usize {
    // The root node is created with player_just_moved set to the opponent so that the
    // current turn is for `player`.
    let mut nodes: Vec<TreeNode> = Vec::new();
    nodes.push(TreeNode::new(root_board, None));
    let mut rng = rand::rng();

    for _ in 0..iterations {
        // 1. **Selection:** Start at the root node.
        let mut node_index = 0;
        let mut path: Vec<usize> = vec![node_index];

        // Traverse the tree until reaching a node that has untried moves or is terminal.
        while !nodes[node_index].board.is_terminal()
            && nodes[node_index].untried_moves.is_empty()
            && !nodes[node_index].children.is_empty()
        {
            let parent_visits = nodes[node_index].visits;
            let mut best_uct = f64::NEG_INFINITY;
            let mut best_child = None;
            for &child_index in &nodes[node_index].children {
                let uct = nodes[child_index].uct_value(parent_visits);
                if uct > best_uct {
                    best_uct = uct;
                    best_child = Some(child_index);
                }
            }
            if let Some(child) = best_child {
                node_index = child;
                path.push(node_index);
            } else {
                break;
            }
        }

        // 2. **Expansion:** If the node is non-terminal and has untried moves, expand one.
        if !nodes[node_index].board.is_terminal() && !nodes[node_index].untried_moves.is_empty() {
            let untried = &mut nodes[node_index].untried_moves;
            let move_index = rng.random_range(0..untried.len());
            let m = untried.swap_remove(move_index); // Remove the chosen move.
                                                     //let current_player = nodes[node_index].player_just_moved.opponent();
            let mut new_board = nodes[node_index].board.clone();

            new_board.make_move(m);
            let new_node = TreeNode::new(new_board, Some(m));
            nodes.push(new_node);
            let new_node_index = nodes.len() - 1;
            nodes[node_index].children.push(new_node_index);
            node_index = new_node_index;
            path.push(node_index);
        }

        // 3. **Simulation:** From the new node, play random moves until reaching a terminal state.
        let mut sim_board = nodes[node_index].board.clone();
        while !sim_board.is_terminal() {
            let legal_moves = sim_board.get_legal_moves();
            if legal_moves.is_empty() {
                break;
            }
            let &m = legal_moves.choose(&mut rng).unwrap();
            sim_board.make_move(m);
        }
        let result = sim_board.check_winner();

        // 4. **Backpropagation:** Update all nodes in the path with the simulation result.
        for &node_idx in path.iter() {
            nodes[node_idx].visits += 1;
            if let Some(winner) = result {
                if winner == nodes[node_idx].board.player {
                    nodes[node_idx].wins += 1.0;
                }
            } else {
                // For a draw, add half a win.
                nodes[node_idx].wins += 0.5;
            }
        }
    }

    // Return the move (from the root's children) with the highest visit count.
    let best_child = nodes[0]
        .children
        .iter()
        .max_by_key(|&&child| nodes[child].visits)
        .expect("There should be at least one child");
    nodes[*best_child].move_played.unwrap()
}
