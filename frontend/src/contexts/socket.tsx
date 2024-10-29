import type { Socket } from "socket.io-client";
import { type ParentProps, createContext, useContext } from "solid-js";

export const SocketContext = createContext<Socket>();

export function SocketIoProvider(props: ParentProps & { socket: Socket }) {
	return (
		<SocketContext.Provider value={props.socket}>
			{props.children}
		</SocketContext.Provider>
	);
}

export function useSocket(): Socket {
	return useContext(SocketContext);
}
