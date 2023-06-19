I am using [Karabiner-Elements](https://karabiner-elements.pqrs.org/) to allow numlock to switch my numpad keys between numbers and custom keys (volume control, arrow keys, macros, script execution). I created a Karabiner variable called `numlock` which is toggled on `key_code: "keypad_num_lock"`. Here is the relevant rule (from `karabiner.json`):

```json
{
	"description": "Rebind numeric keyboard",
	"manipulators": [
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_num_lock"
			},
			"to": [
				{
					"set_variable": {
						"name": "numlock",
						"value": 1
					}
				},
				{
					"shell_command": "/Applications/numlock-tray.app/Contents/Resources/bin/cli enable"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 1
				}
			],
			"from": {
				"key_code": "keypad_num_lock"
			},
			"to": [
				{
					"set_variable": {
						"name": "numlock",
						"value": 0
					}
				},
				{
					"shell_command": "/Applications/numlock-tray.app/Contents/Resources/bin/cli disable"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_0"
			},
			"to": [
				{
					"key_code": "spacebar"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_1"
			},
			"to": [
				{
					"shell_command": "echo 'Execute a script, open a website, run a program...'"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_2"
			},
			"to": [
				{
					"key_code": "down_arrow"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_4"
			},
			"to": [
				{
					"key_code": "left_arrow"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_6"
			},
			"to": [
				{
					"key_code": "right_arrow"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_8"
			},
			"to": [
				{
					"key_code": "up_arrow"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_slash"
			},
			"to": [
				{
					"key_code": "escape"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_hyphen"
			},
			"to": [
				{
					"key_code": "volume_decrement"
				}
			],
			"type": "basic"
		},
		{
			"conditions": [
				{
					"name": "numlock",
					"type": "variable_if",
					"value": 0
				}
			],
			"from": {
				"key_code": "keypad_plus"
			},
			"to": [
				{
					"key_code": "volume_increment"
				}
			],
			"type": "basic"
		}
	]
}
```
