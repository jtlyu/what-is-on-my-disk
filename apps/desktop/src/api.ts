import { invoke } from '@tauri-apps/api/core';
import type {
  Node,
  Scaffold,
  AdvisorRequest,
  AdvisorResponse,
  Plan,
  UndoEntry,
} from './types';

export const api = {
  scan: (path: string) => invoke<Node>('scan_path', { path }),
  listScaffolds: () => invoke<Scaffold[]>('list_scaffolds'),
  detectScaffold: (path: string) => invoke<string | null>('detect_scaffold', { path }),
  advise: (req: AdvisorRequest) => invoke<AdvisorResponse>('advise', { req }),
  inspect: (path: string, sampleCount: number) =>
    invoke<string[]>('inspect_path', { path, sampleCount }),
  execute: (plan: Plan, dryRun: boolean) =>
    invoke<UndoEntry[]>('execute_plan', { plan, dryRun }),
  setAdvisor: (
    provider: 'openai' | 'anthropic' | 'ollama',
    model: string,
    apiKey?: string,
    baseUrl?: string,
  ) =>
    invoke<void>('set_advisor', {
      provider,
      apiKey: apiKey ?? null,
      model,
      baseUrl: baseUrl ?? null,
    }),
};
