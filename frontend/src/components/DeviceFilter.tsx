import {
	Checkbox,
	FormControl,
	InputLabel,
	ListItemText,
	MenuItem,
	OutlinedInput,
	Select,
} from "@suid/material";
import type { SelectChangeEvent } from "@suid/material/Select";
import { For } from "solid-js";

const ITEM_HEIGHT = 48;
const ITEM_PADDING_TOP = 8;
const MenuProps = {
	PaperProps: {
		style: {
			"max-height": `${ITEM_HEIGHT * 4.5 + ITEM_PADDING_TOP}px`,
			width: `${250}px`,
		},
	},
};

const names = [
	"RoCon",
	"RoCon2",
	"GW",
	"G1",
	"HG1",
	"HC1",
	"HCM1",
	"HC2",
	"HCM2",
	"Outdoor",
];

export type DeviceFilterProps = {
	label: string;
	devices: string[];
	setDevices(devices: string[]);
};

export function DeviceFilter(props: DeviceFilterProps) {
	const handleChange = (event: SelectChangeEvent<string | string[]>) => {
		const {
			target: { value },
		} = event;
		props.setDevices(
			// On autofill we get a stringified value.
			typeof value === "string" ? value.split(",") : value,
		);
	};

	return (
		<FormControl sx={{ m: 1, width: 300 }} size="small" style="width: 10em">
			<InputLabel id="device-filter-label">{props.label}</InputLabel>
			<Select
				labelId="device-filter-label"
				id="device-filter-checkbox"
				multiple
				label={props.label}
				value={props.devices}
				onChange={handleChange}
				input={<OutlinedInput label={props.label} />}
				renderValue={(selected) => selected.join(", ")}
				MenuProps={MenuProps}
			>
				<For each={names}>
					{(name) => (
						<MenuItem value={name}>
							<Checkbox checked={props.devices.indexOf(name) > -1} />
							<ListItemText primary={name} />
						</MenuItem>
					)}
				</For>
			</Select>
		</FormControl>
	);
}
