import { io } from "socket.io-client";
import {
	AppBar,
	Box,
	Breadcrumbs,
	Button,
	Drawer,
	IconButton,
	Link,
	List,
	ListItem,
	ListItemButton,
	ListItemIcon,
	ListItemText,
	Paper,
	Toolbar,
	Typography,
} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import { Params } from "./routes/Params";
import { SocketIoProvider } from "./contexts/socket";
import { StateProvider } from "./contexts/state";
import { Route, Router } from "@solidjs/router";
import { CanMonitor } from "./routes/CanMonitor";
import { createSignal } from "solid-js";
import { Portal } from "solid-js/web";
import { Monitor, TableView, Tune } from "@suid/icons-material";
import { CONFIG } from "./config";

export default function App() {
	const socket = io(CONFIG.WEBSOCKET_URL);
	const [showDrawer, setShowDrawer] = createSignal<boolean>(false);
	return (
		<>
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
						<Route path="/can-monitor" component={CanMonitor} />
						<Route path="/params" component={Params} />
					</Router>
				</StateProvider>
			</SocketIoProvider>
		</>
	);
}
