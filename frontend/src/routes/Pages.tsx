import { createResource, For, Show } from "solid-js";
import { useState } from "../contexts/state";
import { ValueDisplay } from "../components/ValueDisplay";
import { apiUrl } from "../config";
import { Portal } from "solid-js/web";
import { Apps, Monitor } from "@suid/icons-material";
import { useCatalog } from "../contexts/catalog";

type Catalog = {
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

export function Pages() {
    let catalog = useCatalog();
    return <>
        <Portal mount={document.getElementById("breadcrumbs")}>
            <Apps style="vertical-align: bottom" sx={{ mr: "0.2em" }} />
            Pages
        </Portal>
        <h1>Configuration</h1>
        {
            JSON.stringify(catalog)
        }
        {/*
        <For each={catalog.pages}>{(page) => (
            <>
                <h2>Page: {page.path}</h2>
                <For each={page.params}>{(param) => (
                    <li>{param.device}.{param.param}</li>
                )}</For>
            </>
        )}</For>
        */}
    </>
}


type ParamProps = {
    device: string,
    param: string,
}
