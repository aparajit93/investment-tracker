import { invoke } from "@tauri-apps/api/core";

export interface CreatePortfolioRequest {
    name: string,
    baseCurrency: string
}

export async function createPortfolio(request: CreatePortfolioRequest) {
    await invoke("create_portfolio", {name: request.name, baseCurrency: request.baseCurrency});
}