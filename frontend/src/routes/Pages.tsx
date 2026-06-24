import { A, useParams } from "@solidjs/router";
import { Apps } from "@suid/icons-material";
import { Alert, Box, Button, Typography } from "@suid/material";
import { createMemo, For, Show } from "solid-js";
import { Portal } from "solid-js/web";
import { PageParamField } from "../components/PageParamField";
import { useCatalog } from "../contexts/catalog";
import { useState } from "../contexts/state";

function formatPageName(path: string) {
	if (path === "") {
		return "Overview";
	}
	const segment = path.split("/").at(-1) || path;
	return segment
		.split("-")
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

function normalizePath(path: string | undefined) {
	if (!path) {
		return "";
	}
	return path.replace(/^\/+|\/+$/g, "");
}

export function Pages() {
	const catalog = useCatalog();
	const state = useState();
	const params = useParams();
	const path = createMemo(() => normalizePath(params.pagePath));
	const currentPage = createMemo(() =>
		catalog.pages.find((page) => page.path === path()),
	);
	const catalogParams = createMemo(
		() => new Map(catalog.params.map((param) => [param.name, param])),
	);
	const subPages = createMemo(() => {
		const currentPath = path();
		const prefix = currentPath === "" ? "" : `${currentPath}/`;
		const depth = currentPath === "" ? 1 : currentPath.split("/").length + 1;
		return catalog.pages.filter(
			(page) =>
				page.path !== currentPath &&
				page.path.startsWith(prefix) &&
				page.path.split("/").length === depth,
		);
	});

	return (
		<>
			<Portal mount={document.getElementById("breadcrumbs")}>
				<Apps style="vertical-align: bottom" sx={{ mr: "0.2em" }} />
				Pages
				<Show when={path() !== ""}> / {formatPageName(path())}</Show>
			</Portal>
			<h1>{formatPageName(path())}</h1>

			<Show
				when={currentPage()}
				fallback={
					<Alert severity="warning">
						Page <code>{path()}</code> was not found in <code>catalog.pages</code>.
					</Alert>
				}
			>
				{(page) => (
					<>
						<Show when={subPages().length > 0}>
							<Typography variant="h6" sx={{ mb: "0.8rem" }}>
								Sub Pages
							</Typography>
							<Box sx={{ display: "flex", gap: "0.6rem", flexWrap: "wrap", mb: "1rem" }}>
								<For each={subPages()}>
									{(subPage) => (
										<Button component={A} href={`/pages/${subPage.path}`} variant="outlined" size="small">
											{formatPageName(subPage.path)}
										</Button>
									)}
								</For>
							</Box>
						</Show>

						<Show when={page().params.length > 0}>
							<Box
								sx={{
									display: "grid",
									gap: "0.8rem",
									gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))",
								}}
							>
								<For each={page().params}>
									{(pageParam) => {
										const catalogParam = catalogParams().get(pageParam.param);
										if (!catalogParam) {
											return (
												<Alert severity="error">
													Missing param metadata for <code>{pageParam.param}</code>.
												</Alert>
											);
										}
										const value = state.devices[pageParam.device]?.[pageParam.param];
										return <PageParamField catalogParam={catalogParam} value={value} />;
									}}
								</For>
							</Box>
						</Show>
					</>
				)}
			</Show>
		</>
	);
}
