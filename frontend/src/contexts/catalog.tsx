import { type ParentProps, Show, createContext, createResource, useContext } from "solid-js";
import { apiUrl } from "../config";

type Catalog = {
    params: string[],
    pages: {
        path: string
        params: [
            {
                device: string,
                param: string,
            }
        ]
    }[]
}

export const CatalogContext = createContext<Catalog>(undefined);

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
    console.log(useContext(CatalogContext));
    return useContext(CatalogContext);
}
