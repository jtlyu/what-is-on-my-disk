import { create } from 'zustand';
import type { Node, Scaffold, AdvisorResponse } from './types';

export interface WalkItem {
  node: Node;
  scaffoldId: string | null;
  advice?: AdvisorResponse;
  status: 'pending' | 'advising' | 'ready' | 'done' | 'skipped';
}

interface AppState {
  root: Node | null;
  scaffolds: Scaffold[];
  selectedPath: string | null;
  walkQueue: WalkItem[];
  walkIndex: number;
  walkThresholdGB: number;
  reclaimedBytes: number;

  setRoot: (n: Node | null) => void;
  setScaffolds: (s: Scaffold[]) => void;
  selectPath: (p: string | null) => void;
  setWalk: (q: WalkItem[]) => void;
  advanceWalk: () => void;
  patchWalkItem: (i: number, patch: Partial<WalkItem>) => void;
  setThreshold: (gb: number) => void;
  addReclaimed: (n: number) => void;
}

export const useStore = create<AppState>((set) => ({
  root: null,
  scaffolds: [],
  selectedPath: null,
  walkQueue: [],
  walkIndex: 0,
  walkThresholdGB: 1,
  reclaimedBytes: 0,

  setRoot: (root) => set({ root }),
  setScaffolds: (scaffolds) => set({ scaffolds }),
  selectPath: (selectedPath) => set({ selectedPath }),
  setWalk: (walkQueue) => set({ walkQueue, walkIndex: 0 }),
  advanceWalk: () => set((s) => ({ walkIndex: Math.min(s.walkIndex + 1, s.walkQueue.length) })),
  patchWalkItem: (i, patch) =>
    set((s) => {
      const q = s.walkQueue.slice();
      q[i] = { ...q[i], ...patch };
      return { walkQueue: q };
    }),
  setThreshold: (walkThresholdGB) => set({ walkThresholdGB }),
  addReclaimed: (n) => set((s) => ({ reclaimedBytes: s.reclaimedBytes + n })),
}));

export function buildWalkQueue(root: Node, thresholdBytes: number): { node: Node; scaffoldId: string | null }[] {
  const out: { node: Node; scaffoldId: string | null }[] = [];
  const visit = (n: Node, depth: number) => {
    if (!n.is_dir) return;
    if (n.size >= thresholdBytes && depth > 0) {
      out.push({ node: n, scaffoldId: n.scaffold_id ?? null });
      return;
    }
    if (depth < 4) {
      for (const c of n.children) visit(c, depth + 1);
    }
  };
  visit(root, 0);
  out.sort((a, b) => b.node.size - a.node.size);
  return out;
}
