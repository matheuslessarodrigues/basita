use sdl2::mixer::{Channel, Music, MAX_VOLUME};

use crate::sdl::SdlStorage;

use super::resources::{Bgms, MusicCommand, MusicCommands, SfxCommands, SfxVariant, Sfxs};

#[derive(Default)]
pub struct Mixer {
	pub sfxs: Sfxs,
	pub bgms: Bgms,
	pub sfx_commands: SfxCommands,
	pub music_commands: MusicCommands,
}

impl Mixer {
	pub fn mix<'a>(&mut self, sdl_storage: &mut SdlStorage<'a>) -> Result<(), String> {
		mix(
			&mut self.sfxs,
			&mut self.bgms,
			&mut self.sfx_commands,
			&mut self.music_commands,
			sdl_storage,
		)
	}
}

pub fn mix<'a>(
	sfxs: &mut Sfxs,
	bgms: &mut Bgms,
	sfx_commands: &mut SfxCommands,
	music_commands: &mut MusicCommands,
	sdl_storage: &mut SdlStorage<'a>,
) -> Result<(), String> {
	for command in sfx_commands.commands.iter() {
		match command.variant {
			SfxVariant::Play => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				let chunk = sdl_storage.chunk_storage.at_mut(sfx.chunk_index);

				if let Ok(channel) = Channel::all().play(chunk, 0) {
					channel.unregister_all_effects()?;
					channel.set_volume(MAX_VOLUME);
					sfx.channel = Some(channel);
				}
			}
			SfxVariant::Volume(volume) => {
				let sfx = sfxs.get(command.sfx_handle);
				if let Some(channel) = sfx.channel {
					channel.set_volume(volume);
				}
			}
			SfxVariant::Pan(left, right) => {
				let sfx = sfxs.get(command.sfx_handle);
				if let Some(channel) = sfx.channel {
					channel.set_panning(left, right)?;
				}
			}
		}
	}
	sfx_commands.commands.clear();

	for command in music_commands.commands.iter() {
		match *command {
			MusicCommand::Play(bgm_handle, fade_in) => {
				let bgm = bgms.get(bgm_handle);
				let music = sdl_storage.music_storage.at(bgm.music_index);
				if let Ok(_) = music.fade_in(0, fade_in) {}
			}
			MusicCommand::Stop(fade_out) => if let Ok(_) = Music::fade_out(fade_out) {},
			MusicCommand::Volume(volume) => Music::set_volume(volume),
		}
	}
	music_commands.commands.clear();

	Ok(())
}
