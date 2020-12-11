use std::{thread, time::Duration};

use rg3d_sound::{
    buffer::{DataSource, SoundBuffer},
    context::Context,
    pool::Handle,
    source::{generic::GenericSourceBuilder, SoundSource},
};

fn main() {
    // Initialize new sound context with default output device.
    let context = Context::new().unwrap();

    // Load sound buffer.
    let footstep_buffer =
        SoundBuffer::new_generic(DataSource::from_file("footstep.wav").unwrap()).unwrap();

    // Create generic source (without spatial effects) using that buffer.
    let source = GenericSourceBuilder::new(footstep_buffer)
        .with_looping(false)
        .build_source()
        .unwrap();

    // Each sound sound must be added to context, context takes ownership on source
    // and returns pool handle to it by which it can be accessed later on if needed.
    let source_handle: Handle<SoundSource> = context.lock().unwrap().add_source(source);
    // This plays the sound way more than once every 5 seconds.
    loop {
        println!("Looping");
        let mut context = context.lock().unwrap();
        let source = context.source_mut(source_handle);
        source.play();
        // This introduces audio artifacts after a while.
        // Use `from_secs(5)` and these go away for me.
        thread::sleep(Duration::from_millis(5000));
    }
}
