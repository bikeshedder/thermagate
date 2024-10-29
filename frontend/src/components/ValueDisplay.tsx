import {
	CheckBoxOutlineBlank,
	CheckBoxOutlined,
	HourglassEmpty,
	NotInterested,
	Refresh,
} from "@suid/icons-material";
import type { ParamValue, Value } from "../schema";

export function valueToString(value: Value) {
	/// FIXME where do the undefined come from?
	if (value === null || value === undefined) {
		return <NotInterested color="disabled" style="vertical-align: bottom" />;
	}
	if ("Bool" in value) {
		return value.Bool ? (
			<CheckBoxOutlined style="vertical-align: bottom" />
		) : (
			<CheckBoxOutlineBlank style="vertical-align: bottom" />
		);
	}
	if ("I8" in value) {
		return value.I8;
	}
	if ("I16" in value) {
		return value.I16;
	}
	if ("I32" in value) {
		return value.I32;
	}
	if ("Dec" in value) {
		return value.Dec;
	}
	if ("Enum8" in value) {
		return value.Enum8[1];
	}
	if ("Enum16" in value) {
		return value.Enum16[1];
	}
	if ("TimeRange" in value) {
		const v = value.TimeRange;
		if (!v) {
			// TODO maybe an icon?
			return "-";
		}
		const start = `${v.start.hour.toString().padStart(2, "0")}:${v.start.minute.toString().padStart(2, "0")}`;
		const end = `${v.end.hour.toString().padStart(2, "0")}:${v.end.minute.toString().padStart(2, "0")}`;
		return `${start} – ${end}`;
	}
	if ("Time" in value) {
		const v = value.Time;
		if (!v) {
			// TODO maybe an icon?
			return "-";
		}
		return `${v.hour.toString().padStart(2, "0")}:${v.minute.toString().padStart(2, "0")}`
	}
	return JSON.stringify(value);
}

export function ValueDisplay(params: { value: Value }) {
	return <>{valueToString(params.value)}</>;
}

export function ParamDisplay(params: { value: ParamValue }) {
	if (params.value === "Loading") {
		return <HourglassEmpty color="disabled" style="vertical-align: bottom" />;
	}
	return <>{valueToString(params.value.Value)}</>;
}
