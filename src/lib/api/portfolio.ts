import { invoke } from "@tauri-apps/api/core";

export interface CreatePortfolioRequest {
    name: string,
    baseCurrency: string
}

export interface Portfolio {
    id: string,
    name: string,
    baseCurrency: string,
    createdAt: string,
    updatedAt: string
}

export async function createPortfolio(request: CreatePortfolioRequest) {
    await invoke("create_portfolio", {name: request.name, baseCurrency: request.baseCurrency});
}

export async function listPortfolios() {
    return await invoke<Portfolio[]>("list_portfolios");
}