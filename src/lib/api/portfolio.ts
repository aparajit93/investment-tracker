import { invoke } from "@tauri-apps/api/core";

export interface Portfolio {
    id: string;
    name: string;
    baseCurrency: string;
    createdAt: string;
    updatedAt: string;
}

export interface CreatePortfolioRequest {
    name: string;
    baseCurrency: string;
}

export interface RenamePortfolioRequest {
    id: string;
    name: string;
}

export interface ChangePortfolioBaseCurrencyRequest {
    id: string;
    baseCurrency: string;
}

export async function createPortfolio(request: CreatePortfolioRequest) {
    await invoke("create_portfolio", {name: request.name, baseCurrency: request.baseCurrency});
}

export async function listPortfolios() {
    return await invoke<Portfolio[]>("list_portfolios");
}

export async function renamePortfolio(request:RenamePortfolioRequest) {
    await invoke("rename_portfolio", {id: request.id, name: request.name});
}

export async function changePortfolioBaseCurrency(request:ChangePortfolioBaseCurrencyRequest) {
    await invoke("change_portfolio_base_currency", {id: request.id, baseCurrency: request.baseCurrency});
}

export async function deletePortfolio(id:string) {
    await invoke("delete_portfolio", { id });
}

export async function getPortfolio(id:string) {
    return await invoke<Portfolio>("get_portfolio", { id })
}