import { type ParentProps, Show, createContext, createResource, useContext } from "solid-js";
import { apiUrl } from "../config";

type Unit = "C" | "K" | "kW" | "kWh" | "l/h" | "bar" | "h" | "%" | "A" | "min" | "s";

export type MultilingualString = {
	de: string;
	en: string;
};

type NumberType = {
	default: number | null;
	unit: Unit | null;
	min: number | null;
	max: number | null;
};

type EnumType = {
	default: EnumVariant | null;
	variants: EnumVariant[];
};

type EnumVariant = {
	value: number;
	code: string;
	label: MultilingualString;
};

export type CatalogParam = {
	id: number;
	name: string;
	label: MultilingualString;
	read: boolean;
	write: boolean;
	type:
		| {
				Bool: {
					default: boolean | null;
				};
		  }
		| {
				I8: NumberType;
		  }
		| {
				I16: NumberType;
		  }
		| {
				Dec: {
					default: string | null;
					unit: Unit | null;
					scale: number;
					min: string | null;
					max: string | null;
				};
		  }
		| {
				Enum8: EnumType;
		  }
		| {
				Enum16: EnumType;
		  }
		| {
				Time: {
					default: {
						hour: number;
						minute: number;
					} | null;
				};
		  }
		| {
				TimeRange: {
					default: {
						start: { hour: number; minute: number };
						end: { hour: number; minute: number };
					} | null;
				};
		  };
};

export type CatalogPage = {
	path: string;
	params: {
		device: string;
		param: string;
	}[];
};

export type Catalog = {
	params: CatalogParam[];
	pages: CatalogPage[];
};

export const CatalogContext = createContext<Catalog | undefined>(undefined);

export function CatalogProvider(props: ParentProps) {
    let [catalog] = createResource<Catalog>(async () => (await fetch(apiUrl("catalog"))).json())
    return (
        <Show when={catalog()} fallback={"Catalog loading..."}>
            <CatalogContext.Provider value={catalog()}>
                {props.children}
            </CatalogContext.Provider>
        </Show>
    );
}

export function useCatalog(): Catalog {
	const catalog = useContext(CatalogContext);
	if (!catalog) {
		throw new Error("Catalog context is not available");
	}
	return catalog;
}
