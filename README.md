# battery_notifier
A small deamon which send a notification if the battery level reach a certain value. (For linux, use libnotify)

## Usage:
Run the following command to install:
```sh
./Install.sh
```

Put the following command in order to execute the program when login on the session:
```sh
battery_notifier <PERCENTAGE>
# <PERCENTAGE> is the percentage of battery when the notification should be sent.
```
### Example for i3:
In i3/config put:
```sh
exec --no-startup-id battery_notifier <PERCENTAGE>
```
