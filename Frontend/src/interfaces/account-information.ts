import { Balance } from "./balance";
import { CommissionRates } from "./commission-rates";

export interface AccountInformation {
    makerCommission: number;
    takerCommission: number;
    buyerCommission: number;
    sellerCommission: number;

    commissionRates: CommissionRates;

    canTrade: boolean;
    canWithdraw: boolean;
    canDeposit: boolean;

    brokered: boolean;
    requireSelfTradePrevention: boolean;
    preventSor: boolean;

    updateTime: number;
    accountType: string;

    balances: Balance[];
    permissions: string[];

    uid: number;
}
