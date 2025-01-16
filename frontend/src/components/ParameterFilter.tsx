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
import { createMemo, createResource, For } from "solid-js";
import { apiUrl } from "../config";

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

export type ParameterFilterProps = {
    label: string;
    parameters: string[];
    setParameters(devices: string[]);
};

type Config = {
    params: {
        name: string
    }[]
}

export function ParameterFilter(props: ParameterFilterProps) {
    let [masterData] = createResource<Config>(async () => (await fetch(apiUrl("master_data"))).json())
    let params = createMemo(() => {
        return (masterData()?.params ?? []).toSorted((a, b) => {
            if (a.name < b.name) { return -1 }
            if (a.name > b.name) { return 1 }
            return 0
        })
    })

    const handleChange = (event: SelectChangeEvent<string | string[]>) => {
        const {
            target: { value },
        } = event;
        props.setParameters(
            // On autofill we get a stringified value.
            typeof value === "string" ? value.split(",") : value,
        );
    };

    return (
        <FormControl sx={{ m: 1, width: 300 }} size="small" style="width: 10em">
            <InputLabel id="parameter-filter-label">{props.label}</InputLabel>
            <Select
                labelId="parameter-filter-label"
                id="parameter-filter-checkbox"
                multiple
                label={props.label}
                value={props.parameters}
                onChange={handleChange}
                input={<OutlinedInput label={props.label} />}
                renderValue={(selected) => selected.join(", ")}
                MenuProps={MenuProps}
            >
                <For each={params()}>
                    {(param) => (
                        <MenuItem value={param.name}>
                            <Checkbox checked={props.parameters.indexOf(param.name) > -1} />
                            <ListItemText primary={param.name} />
                        </MenuItem>
                    )}
                </For>
            </Select>
        </FormControl>
    );
}
