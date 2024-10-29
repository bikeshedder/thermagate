import { createMemo, createResource, createSignal, For, Show } from "solid-js";
import { useState } from "../contexts/state";
import { ParamDisplay } from "../components/ValueDisplay";
import type { ParamValue } from "../schema";
import { Portal } from "solid-js/web";
import {
	alpha,
	Box,
	Card,
	CardContent,
	InputBase,
	Paper,
	styled,
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableRow,
	TextField,
	Typography,
} from "@suid/material";
import { Search as SearchIcon, Tune } from "@suid/icons-material";

function entriesSorted<T>(obj: Record<string, T>): Array<[string, T]> {
	const keys = Object.keys(obj);
	keys.sort();
	const entries = [];
	for (const key of keys) {
		entries.push([key, obj[key]]);
	}
	return entries;
}

const Search = styled("div")(({ theme }) => ({
	position: "relative",
	borderRadius: theme.shape.borderRadius,
	backgroundColor: alpha(theme.palette.common.white, 0.15),
	"&:hover": {
		backgroundColor: alpha(theme.palette.common.white, 0.25),
	},
	marginLeft: 0,
	width: "100%",
	[theme.breakpoints.up("sm")]: {
		marginLeft: theme.spacing(1),
		width: "auto",
	},
}));

const SearchIconWrapper = styled("div")(({ theme }) => ({
	padding: theme.spacing(0, 2),
	height: "100%",
	position: "absolute",
	pointerEvents: "none",
	display: "flex",
	alignItems: "center",
	justifyContent: "center",
}));

const StyledInputBase = styled(InputBase)(({ theme }) => ({
	color: "inherit",
	width: "100%",
	"& .MuiInputBase-input": {
		padding: theme.spacing(1, 1, 1, 0),
		// vertical padding + font size from searchIcon
		paddingLeft: `calc(1em + ${theme.spacing(4)})`,
		transition: theme.transitions.create("width"),
		[theme.breakpoints.up("sm")]: {
			width: "12ch",
			"&:focus": {
				width: "20ch",
			},
		},
	},
}));

export function Params() {
	const state = useState();
	const [search, setSearch] = createSignal("");
	const lcSearch = createMemo(() => search().toLowerCase().trim());
	function paramStyle(name: string) {
		const s = lcSearch();
		if (s === "" || name.toLowerCase().includes(s)) {
			return undefined;
		}
		return {
			display: "none",
		};
	}
	function deviceStyle(device: Record<string, ParamValue>) {
		const s = lcSearch();
		if (
			s === "" ||
			Object.keys(device).find((n) => n.toLowerCase().includes(lcSearch())) !==
				undefined
		) {
			return undefined;
		}
		return {
			display: "none",
		};
	}
	return (
		<>
			<Portal mount={document.getElementById("portal")}>
				<Search>
					<SearchIconWrapper>
						<SearchIcon />
					</SearchIconWrapper>
					<StyledInputBase
						placeholder="Search…"
						inputProps={{ "aria-label": "search" }}
						value={search()}
						onChange={(ev) => setSearch(ev.target.value)}
					/>
				</Search>
			</Portal>
			<Portal mount={document.getElementById("breadcrumbs")}>
				<Tune style="vertical-align: bottom" sx={{ mr: "0.2em" }} />
				Parameters
			</Portal>
			<Show when={state.devices}>
				{(devices) => (
					<For each={Object.entries(devices())}>
						{([deviceName, device]) => (
							<Box sx={{ my: "1rem" }} style={deviceStyle(device)}>
								<Card>
									<CardContent>
										<Typography variant="h5">{deviceName}</Typography>
									</CardContent>
									<Table size="small">
										<TableHead>
											<TableRow>
												<TableCell component="th" style={{ width: "20em" }}>
													Name
												</TableCell>
												<TableCell component="th">Value</TableCell>
											</TableRow>
										</TableHead>
										<TableBody>
											<For each={Object.entries(device)}>
												{([paramName, value]) => (
													<TableRow style={paramStyle(paramName)}>
														<TableCell>{paramName}</TableCell>
														<TableCell>
															<ParamDisplay value={value} />
														</TableCell>
													</TableRow>
												)}
											</For>
										</TableBody>
									</Table>
								</Card>
							</Box>
						)}
					</For>
				)}
			</Show>
		</>
	);
}
