import { PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";



// Enums (mirror on-chain)

export enum EmotionalState {
  Neutral = 0,
  Confident = 1,
  Fearful = 2,
  Greedy = 3,
  Anxious = 4,
  Calm = 5,
  Frustrated = 6,
  Euphoric = 7,
  Revenge = 8,
}

export const EMOTIONAL_STATE_LABELS: Record<EmotionalState, string> = {
  [EmotionalState.Neutral]: "Neutral",
  [EmotionalState.Confident]: "Confident",
  [EmotionalState.Fearful]: "Fearful",
  [EmotionalState.Greedy]: "Greedy",
  [EmotionalState.Anxious]: "Anxious",
  [EmotionalState.Calm]: "Calm",
  [EmotionalState.Frustrated]: "Frustrated",
  [EmotionalState.Euphoric]: "Euphoric",
  [EmotionalState.Revenge]: "Revenge",
};

export enum TradeDirection {
  Long = 0,
  Short = 1,
}

export enum AssetClass {
  Spot = 0,
  Perpetual = 1,
  Options = 2,
  Futures = 3,
  Other = 4,
}

export const ASSET_CLASS_LABELS: Record<AssetClass, string> = {
  [AssetClass.Spot]: "Spot",
  [AssetClass.Perpetual]: "Perpetual",
  [AssetClass.Options]: "Options",
  [AssetClass.Futures]: "Futures",
  [AssetClass.Other]: "Other",
};

export enum TradeStatus {
  Open = 0,
  Closed = 1,
}


// Account structures (deserialized from chain)

export interface TradingStats {
  totalTrades: number;
  winningTrades: number;
  totalPnlRealized: BN; // i64 micro-units
  bestTradePnl: BN;
  worstTradePnl: BN;
  totalFeesPaid: BN;
  currentWinStreak: number;
  longestWinStreak: number;
  currentLoseStreak: number;
  longestLoseStreak: number;
}

export interface UserAccount {
  version: number;
  owner: PublicKey;
  bump: number;
  username: string;
  bio: string;
  createdAt: BN;
  tradeCount: number;
  tagCount: number;
  stats: TradingStats;
  privacyEnabled: boolean;
}

export interface TradeAccount {
  version: number;
  owner: PublicKey;
  bump: number;
  tradeIndex: number;
  symbol: string;
  direction: TradeDirection;
  assetClass: AssetClass;
  entryPrice: BN;
  exitPrice: BN;
  size: BN;
  leverage: number;
  pnlRealized: BN;
  feesPaid: BN;
  entryTimestamp: BN;
  exitTimestamp: BN;
  emotionEntry: EmotionalState;
  emotionExit: EmotionalState;
  notes: string;
  tagIndices: number[];
  status: TradeStatus;
  updatedAt: BN;
  // Added by client for display
  publicKey?: PublicKey;
}

export interface TagAccount {
  owner: PublicKey;
  bump: number;
  tagIndex: number;
  name: string;
  color: number;
  usageCount: number;
  createdAt: BN;
  publicKey?: PublicKey;
}



// Frontend-only helpers

/** Convert micro-unit i64 to human-readable dollars */
export function microToUsd(micro: BN | number): number {
  const val = typeof micro === "number" ? micro : micro.toNumber();
  return val / 1_000_000;
}

/** Convert dollar amount to micro-units for on-chain storage */
export function usdToMicro(usd: number): number {
  return Math.round(usd * 1_000_000);
}

export function formatPnl(micro: BN | number): string {
  const usd = microToUsd(micro);
  const sign = usd >= 0 ? "+" : "";
  return `${sign}$${usd.toFixed(2)}`;
}

export function winRate(stats: TradingStats): number {
  if (stats.totalTrades === 0) return 0;
  return (stats.winningTrades / stats.totalTrades) * 100;
}

export interface CreateTradeFormData {
  symbol: string;
  direction: TradeDirection;
  assetClass: AssetClass;
  entryPriceUsd: number;
  sizeBase: number;
  leverage: number;
  emotionEntry: EmotionalState;
  notes: string;
  tagIndices: number[];
  entryTimestamp: number;
}

export interface CloseTradeFormData {
  exitPriceUsd: number;
  feesUsd: number;
  emotionExit: EmotionalState;
  exitTimestamp: number;
}