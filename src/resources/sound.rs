use bevy::prelude::*;

#[derive(Resource,Clone,Debug)]
pub struct MySound{
    pub sound: Handle<AudioSource>,
}




pub fn setup_sound_resources(mut commands: Commands, asset_server: Res<AssetServer>,) {
    let sound: Handle<AudioSource> = asset_server.load("cat/nyaa.mp3");
    commands.insert_resource(
        MySound{
            sound,
        },
    );
}