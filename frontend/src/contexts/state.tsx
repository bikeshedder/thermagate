import { type ParentProps, createContext, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { type ParamValue, ParamMessage, ParamsMessage } from "../schema";
import { useSocket } from "./socket";

export const StateContext = createContext<State>();

export type State = {
	devices: Record<string, Record<string, ParamValue>>;
};

export function StateProvider(props: ParentProps) {
	const [params, setParams] = createStore<State>({
		devices: {},
	});
	const socket = useSocket();
	socket.on("params", (rawMessage) => {
		const res = ParamsMessage.safeParse(rawMessage);
		if (!res.success) {
			console.log("Unsupported message: ", rawMessage, res.error);
			return;
		}
		setParams({ devices: res.data });
	});
	socket.on("param", (rawMessage) => {
		const res = ParamMessage.safeParse(rawMessage);
		if (!res.success) {
			console.log("Unsupported message: ", rawMessage);
			return;
		}
		const message = res.data;
		if (message.device.name !== "Other") {
			setParams("devices", message.device.name, (v) => v || {});
			setParams(
				"devices",
				message.device.name,
				message.param.name || `Other(${message.param.id})`,
				message.value,
			);
		}
	});
	return (
		<StateContext.Provider value={params}>
			{props.children}
		</StateContext.Provider>
	);
}

export function useState(): State {
	return useContext(StateContext);
}
