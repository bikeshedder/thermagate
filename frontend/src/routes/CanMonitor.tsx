import { Delete, Monitor, Pause, PlayArrow, PortraitSharp } from "@suid/icons-material";
import {
	Box,
	Button,
	Card,
	Chip,
	Fab,
	IconButton,
	Paper,
	Table,
	TableBody,
	TableCell,
	TableContainer,
	TableHead,
	TableRow,
	Typography,
} from "@suid/material";
import { For, Show, createSignal, onCleanup } from "solid-js";
import { Portal } from "solid-js/web";
import { DeviceFilter } from "../components/DeviceFilter";
import { ValueDisplay } from "../components/ValueDisplay";
import { useSocket } from "../contexts/socket";
import { CanMessage, Device, type MessageType, Value } from "../schema";

const MESSAGE_TYPE_COLORS: Record<
	MessageType,
	"default" | "primary" | "secondary" | "error" | "info" | "success" | "warning"
> = {
	Request: "primary",
	Response: "secondary",
	Set: "error",
	Ping: "warning",
	Pong: "warning",
	Other: "default",
};

function DeviceName(props: { device: Device }) {
	return <Show
		when={props.device.name != "Other"}
		fallback={<code>{props.device.id.toString(16).padStart(4, "0")}</code>}>
			{props.device.name}
	</Show>
}

export function CanMonitor() {
	const socket = useSocket();
	const [messages, setMessages] = createSignal<CanMessage[]>([]);
	const [pause, setPause] = createSignal<boolean>(false);
	const [filterSender, setFilterSender] = createSignal<string[]>([]);
	const [filterReceiver, setFilterReceiver] = createSignal<string[]>([]);
	const canListener = (rawMessage) => {
		const res = CanMessage.safeParse(rawMessage);
		if (!res.success) {
			console.log("Unsupported messaeg:", rawMessage);
			return;
		}
		const msg = res.data;
		if (pause()) {
			return;
		}
		if (filterSender().length) {
			if (!filterSender().includes(msg.sender.name)) {
				return;
			}
		}
		if (filterReceiver().length) {
			if (!filterReceiver().includes(msg.receiver.name)) {
				return;
			}
		}
		const msgs = messages();
		setMessages([...msgs.slice(-1000), msg]);
		window.scroll(0, document.body.scrollHeight);
	};
	socket.on("can", canListener);
	onCleanup(() => {
		socket.off("can", canListener);
	});
	return (
		<>
			<Fab
				aria-label="Continue/Pause"
				onClick={() => setPause(!pause())}
				sx={{ position: "fixed", bottom: 16, right: 16 }}
			>
				<Show when={pause()}>
					<PlayArrow />
				</Show>
				<Show when={!pause()}>
					<Pause />
				</Show>
			</Fab>
			<Portal mount={document.getElementById("breadcrumbs")}>
				<Monitor style="vertical-align: bottom" sx={{ mr: "0.2em" }} />
				CAN monitor
			</Portal>
			<Portal mount={document.getElementById("portal")}>
				<Card>
					<IconButton onClick={() => setMessages([])} size="large" sx={{ p: 2 }} color="error"><Delete/></IconButton>
					<DeviceFilter
						label="Sender"
						devices={filterSender()}
						setDevices={setFilterSender}
					/>
					<DeviceFilter
						label="Receiver"
						devices={filterReceiver()}
						setDevices={setFilterReceiver}
					/>
				</Card>
			</Portal>
			<TableContainer component={Paper}>
				<Table size="small">
					<TableHead>
						<TableRow>
							<TableCell component="th">Raw</TableCell>
							<TableCell component="th">Sender</TableCell>
							<TableCell component="th">Receiver</TableCell>
							<TableCell component="th">Type</TableCell>
							<TableCell component="th">Parameter</TableCell>
							<TableCell component="th">Value</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						<For each={messages()}>
							{(msg) => (
								<TableRow
									sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
								>
									<TableCell>
										<code style="white-space: nowrap">
											{msg.raw
												.map((n) => n.toString(16).padStart(2, "0"))
												.join(" ")}
										</code>
									</TableCell>
									<TableCell><DeviceName device={msg.sender}/></TableCell>
									<TableCell><DeviceName device={msg.receiver}/></TableCell>
									<TableCell>
										<Chip
											color={MESSAGE_TYPE_COLORS[msg.type]}
											label={msg.type}
										/>
									</TableCell>
									<TableCell>
										{msg.param.name ||
											msg.param.id.toString(16).padStart(4, "0")}
									</TableCell>
									<TableCell>
										<Show when={msg.type === "Response" || msg.type === "Set"}>
											<ValueDisplay value={msg.value} />
										</Show>
									</TableCell>
								</TableRow>
							)}
						</For>
					</TableBody>
				</Table>
			</TableContainer>
		</>
	);
}
