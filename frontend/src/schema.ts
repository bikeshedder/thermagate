import z from "zod";

export const Param = z.object({
    id: z.number(),
    name: z.string(),
})
export type Param = z.infer<typeof Param>

export const CanParam = z.object({
    id: z.number(),
    name: z.string().nullable(),
})
export type CanParam = z.infer<typeof Param>

export const CanDevice = z.object({
    id: z.number(),
    name: z.string().nullable(),
})
export type CanDevice = z.infer<typeof Device>

export const Device = z.object({
    id: z.number(),
    name: z.string(),
})
export type Device = z.infer<typeof Device>

export const MessageType = z.enum([
    "Request",
    "Response",
    "Set",
    "Ping",
    "Pong",
    "Other"
])
export type MessageType = z.infer<typeof MessageType>

export const Time = z.object({
    hour: z.number(),
    minute: z.number(),
})
export type Time = z.infer<typeof Time>

export const Value = z.union([
    z.object({
        Bool: z.boolean().nullable(),
    }),
    z.object({
        I8: z.number(),
    }),
    z.object({
        I16: z.number(),
    }),
    z.object({
        I32: z.number(),
    }),
    z.object({
        // FIXME convert to BigDecimal?
        Dec: z.string(),
    }),
    z.object({
        Enum8: z.tuple([z.number(), z.string().nullable()]),
    }),
    z.object({
        Enum16: z.tuple([z.number(), z.string().nullable()]),
    }),
    z.object({
        Time: Time
    }),
    z.object({
        TimeRange: z.object({
            start: Time,
            end: Time,
        })
    }),
]).nullable();
export type Value = z.infer<typeof Value>

export const ParamValue = z.union([
    z.object({ Value: Value }),
    z.literal("Loading"),
])
export type ParamValue = z.infer<typeof ParamValue>

export const CanMessage = z.object({
    raw: z.array(z.number()),
    sender: CanDevice,
    receiver: CanDevice,
    type: MessageType,
    param: CanParam,
    value: Value.optional(),
})
export type CanMessage = z.infer<typeof CanMessage>


export const ParamMessage = z.object({
    device: Device,
    param: Param,
    value: ParamValue,
})
export type ParamMessage = z.infer<typeof ParamMessage>


export const ParamsMessage = z.record(z.record(ParamValue))
export type ParamsMessage = z.infer<typeof ParamsMessage>
