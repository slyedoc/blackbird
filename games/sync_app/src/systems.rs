use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use rand::prelude::*;

pub fn change_name(mut query: Query<&mut Name, With<Transform>>, mut rng: GlobalEntropy<WyRand>) {
    query
        .single_mut()
        .expect("Name not found")
        .set(format!("Changed {}", rng.next_u32()));
}
