import { io } from "socket.io-client";
import {
	AppBar,
	Breadcrumbs,
	Divider,
	Drawer,
	IconButton,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Toolbar,
	Typography,
} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import { Params } from "./routes/Params";
import { SocketIoProvider } from "./contexts/socket";
import { StateProvider } from "./contexts/state";
import { Route, Router } from "@solidjs/router";
import { CanMonitor } from "./routes/CanMonitor";
import { Pages } from "./routes/Pages";
import { createSignal, For } from "solid-js";
import { Monitor, Tune, Settings } from "@suid/icons-material";
import { CONFIG } from "./config";
import { CatalogProvider, useCatalog } from "./contexts/catalog";

export default function App() {
	const socket = io(CONFIG.WEBSOCKET_URL);
	const [showDrawer, setShowDrawer] = createSignal<boolean>(false);
	return (
		<>
			<CatalogProvider>
				<SocketIoProvider socket={socket}>
					<StateProvider>
						<AppBar>
							<Toolbar>
								<IconButton
									size="large"
									edge="start"
									color="inherit"
									aria-label="menu"
									sx={{ mr: 2 }}
									onClick={() => setShowDrawer(!showDrawer())}
								>
									<MenuIcon />
								</IconButton>
								<Typography variant="h6" component="div" sx={{ mr: "2em" }}>
									Thermagate
								</Typography>
								<Breadcrumbs
									sx={{ flexGrow: 1 }}
									id="breadcrumbs"
									style={{ color: "inherit" }}
								/>
								<div id="portal" />
							</Toolbar>
						</AppBar>
						<Drawer open={showDrawer()} onClose={() => setShowDrawer(false)}>
							<List>
								<PagesMenu setShowDrawer={setShowDrawer} />
								<Divider />
								<ListItem disablePadding>
									<ListItemButton<"a">
										component="a"
										href="/can-monitor"
										onClick={() => setShowDrawer(false)}
									>
										<ListItemIcon>
											<Monitor />
										</ListItemIcon>
										<ListItemText primary="CAN monitor" />
									</ListItemButton>
								</ListItem>
								<ListItem disablePadding>
									<ListItemButton<"a">
										component="a"
										href="/params"
										onClick={() => setShowDrawer(false)}
									>
										<ListItemIcon>
											<Tune />
										</ListItemIcon>
										<ListItemText primary="Parameters" />
									</ListItemButton>
								</ListItem>
							</List>
						</Drawer>
						{/* FIXME this is silly */}
						<h1>&nbsp;</h1>
						<Router>
							<Route path="/pages" component={Pages} />
							<Route path="/pages/*pagePath" component={Pages} />
							<Route path="/can-monitor" component={CanMonitor} />
							<Route path="/params" component={Params} />
						</Router>
					</StateProvider>
				</SocketIoProvider>
			</CatalogProvider>
		</>
	);
}

function pageTitle(path: string) {
	if (path === "") {
		return "Overview";
	}
	const segment = path.split("/").at(-1) || path;
	return segment
		.split("-")
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

function PagesMenu(props: { setShowDrawer: (showDrawer: boolean) => void }) {
	const catalog = useCatalog();
	return (
		<>
			<ListItem disablePadding>
				<ListItemButton<"a">
					component="a"
					href="/pages"
					onClick={() => props.setShowDrawer(false)}
				>
					<ListItemIcon>
						<Settings />
					</ListItemIcon>
					<ListItemText primary="Pages" secondary="Overview" />
				</ListItemButton>
			</ListItem>
			<For each={catalog.pages.filter((page) => page.path !== "")}>
				{(page) => {
					const depth = page.path.split("/").length;
					return (
						<ListItem disablePadding>
							<ListItemButton<"a">
								component="a"
								href={`/pages/${page.path}`}
								sx={{ pl: `${0.8 + depth * 1.3}rem` }}
								onClick={() => props.setShowDrawer(false)}
							>
								<ListItemIcon sx={{ "min-width": "2rem" }}>
									<Settings fontSize="small" />
								</ListItemIcon>
								<ListItemText primary={pageTitle(page.path)} secondary={page.path} />
							</ListItemButton>
						</ListItem>
					);
				}}
			</For>
		</>
	);
}
