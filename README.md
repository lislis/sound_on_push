## sound_on_push

Small program to read an arcade button and play a sound file if done so.

To be compiled for and run on a Raspberry Pi.

### Debugging ALSA

It's possible that your PI's default audio out is HDMI, then you won't hear the audio or you might just get an error altogether.

You can check the order of output devices here:

```
$ cat /proc/asound/cards
```
If headphones is _not_ index 0, you need to make it the default.

```
$ touch ~/.asoundrc
```
Then paste these two lines into `~/.asoundrc`, replacing 1 with the index of the headphones you got from `cat /proc/asound/cards` (likely 1 tho)
```
defaults.pcm.card 1
defaults.ctl.card 1
```
And restart the systemd service
