import { invoke } from "@tauri-apps/api/core";

export enum AccountType {
    Brokerage = "Brokerage",
    Chequing = "Chequing",
    Cash = "Cash",
    Savings = "Savings",
}

export interface Account {
    id: string;
    portfolioId: string;
    name: string;
    institution: string|null;
    accountType: AccountType;
    currency: string;
    isActive: boolean;
    createdAt: string;
    updatedAt: string;
}

export interface CreateAccountRequest {
    portfolioId: string;
    name: string;
    institution: string|null;
    accountType: AccountType;
    currency: string;
}

export interface RenameAccountRequest {
    id: string;
    name: string;
}

export interface ChangeAccountCurrencyRequest {
    id: string;
    currency: string;
}

export interface ChangeAccountInstitutionRequest {
    id: string;
    institution: string|null;
}

export interface ChangeAccountTypeRequest {
    id: string;
    accountType: AccountType;
}

export async function createAccount(request:CreateAccountRequest) {
    await invoke("create_account", { 
        portfolioId: request.portfolioId, 
        name: request.name, 
        institution: request.institution, 
        accountType: request.accountType, 
        currency: request.currency
    });
}

export async function listAccounts(portfolioId:string) {
    return await invoke<Account[]>("list_accounts", { portfolioId: portfolioId });
}

export async function renameAccount(request:RenameAccountRequest) {
    await invoke("rename_account", { id: request.id, name: request.name });
}

export async function changeAccountCurrency(request:ChangeAccountCurrencyRequest) {
    await invoke("change_account_currency", { id: request.id, currency: request.currency });
}

export async function changeAccountInstitution(request:ChangeAccountInstitutionRequest) {
    await invoke("change_account_institution", { id: request.id, institution: request.institution });
}

export async function changeAccountType(request:ChangeAccountTypeRequest) {
    await invoke("change_account_type", { id: request.id, accountType: request.accountType });
}

export async function activateAccount(id:string) {
    await invoke("activate_account", { id: id });
}

export async function deactivateAccount(id:string) {
    await invoke("deactivate_account", { id: id });
}

export async function deleteAccount(id:string) {
    await invoke("delete_account", { id });
}