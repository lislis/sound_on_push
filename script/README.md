This is meant to run on a Raspberry Pi on boot.

For that I prepared a Systemd service. Please note paths might be different if you clone and build yourself or just copy the binary to places.

Copy the .service file to /lib/systemd/system/

`$ cp ./sound_on_push.service /lib/systemd/system/sound_on_push.service`

Modify the permissions

`sudo chmod 644 /lib/systemd/system/sound_on_push.service`

And run on boot

`sudo systemctl enable sound_on_push.service`

You can also start the system rigt away with:

`sudo systemctl start sound_on_push.service`
