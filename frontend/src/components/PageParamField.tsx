import { Box, Card, CardContent, Chip, Typography } from "@suid/material";
import { CheckBoxOutlineBlank, CheckBoxOutlined } from "@suid/icons-material";
import type { JSX } from "solid-js";
import type { CatalogParam, MultilingualString } from "../contexts/catalog";
import type { ParamValue, Value } from "../schema";
import { ParamDisplay, ValueDisplay } from "./ValueDisplay";

type PageParamFieldProps = {
	catalogParam: CatalogParam;
	value: ParamValue | undefined;
};

function labelText(label: MultilingualString) {
	return label.en || label.de || "";
}

function meta(catalogParam: CatalogParam) {
	return (
		<Box sx={{ display: "flex", gap: "0.4rem", mb: "0.4rem" }}>
			<Chip size="small" label={catalogParam.read ? "read" : "no-read"} />
			<Chip size="small" label={catalogParam.write ? "write" : "no-write"} />
		</Box>
	);
}

function title(catalogParam: CatalogParam) {
	return (
		<>
			<Typography variant="h6">{labelText(catalogParam.label)}</Typography>
			<Typography color="text.secondary" sx={{ mb: "0.4rem" }}>
				{catalogParam.name}
			</Typography>
		</>
	);
}

function content(catalogParam: CatalogParam, body: JSX.Element) {
	return (
		<Card>
			<CardContent>
				{title(catalogParam)}
				{meta(catalogParam)}
				{body}
			</CardContent>
		</Card>
	);
}

function valueOrLoading(rawValue: ParamValue | undefined): ParamValue {
	return rawValue || "Loading";
}

function plainValue(rawValue: ParamValue | undefined): Value | null {
	const value = valueOrLoading(rawValue);
	return value === "Loading" ? null : value.Value;
}

function BoolParamField(props: PageParamFieldProps) {
	const value = plainValue(props.value);
	if (value && "Bool" in value && value.Bool !== null) {
		return content(
			props.catalogParam,
			<Box sx={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
				{value.Bool ? (
					<CheckBoxOutlined style="vertical-align: bottom" />
				) : (
					<CheckBoxOutlineBlank style="vertical-align: bottom" />
				)}
				<Typography>{value.Bool ? "Enabled" : "Disabled"}</Typography>
			</Box>,
		);
	}
	return content(props.catalogParam, <ParamDisplay value={valueOrLoading(props.value)} />);
}

function I8ParamField(props: PageParamFieldProps) {
	const unit = "I8" in props.catalogParam.type ? props.catalogParam.type.I8.unit : null;
	return content(
		props.catalogParam,
		<Typography>
			<ParamDisplay value={valueOrLoading(props.value)} />
			{unit ? ` ${unit}` : ""}
		</Typography>,
	);
}

function I16ParamField(props: PageParamFieldProps) {
	const unit = "I16" in props.catalogParam.type ? props.catalogParam.type.I16.unit : null;
	return content(
		props.catalogParam,
		<Typography>
			<ParamDisplay value={valueOrLoading(props.value)} />
			{unit ? ` ${unit}` : ""}
		</Typography>,
	);
}

function DecParamField(props: PageParamFieldProps) {
	const unit = "Dec" in props.catalogParam.type ? props.catalogParam.type.Dec.unit : null;
	return content(
		props.catalogParam,
		<Typography>
			<ParamDisplay value={valueOrLoading(props.value)} />
			{unit ? ` ${unit}` : ""}
		</Typography>,
	);
}

function Enum8ParamField(props: PageParamFieldProps) {
	return content(props.catalogParam, <ParamDisplay value={valueOrLoading(props.value)} />);
}

function Enum16ParamField(props: PageParamFieldProps) {
	return content(props.catalogParam, <ParamDisplay value={valueOrLoading(props.value)} />);
}

function TimeParamField(props: PageParamFieldProps) {
	return content(props.catalogParam, <ParamDisplay value={valueOrLoading(props.value)} />);
}

function TimeRangeParamField(props: PageParamFieldProps) {
	return content(props.catalogParam, <ParamDisplay value={valueOrLoading(props.value)} />);
}

function UnknownParamField(props: PageParamFieldProps) {
	return content(
		props.catalogParam,
		<Typography>
			<ValueDisplay value={plainValue(props.value)} />
		</Typography>,
	);
}

export function PageParamField(props: PageParamFieldProps) {
	const p = props.catalogParam;
	if ("Bool" in p.type) {
		return <BoolParamField {...props} />;
	}
	if ("I8" in p.type) {
		return <I8ParamField {...props} />;
	}
	if ("I16" in p.type) {
		return <I16ParamField {...props} />;
	}
	if ("Dec" in p.type) {
		return <DecParamField {...props} />;
	}
	if ("Enum8" in p.type) {
		return <Enum8ParamField {...props} />;
	}
	if ("Enum16" in p.type) {
		return <Enum16ParamField {...props} />;
	}
	if ("Time" in p.type) {
		return <TimeParamField {...props} />;
	}
	if ("TimeRange" in p.type) {
		return <TimeRangeParamField {...props} />;
	}
	return <UnknownParamField {...props} />;
}
