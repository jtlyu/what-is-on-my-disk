import { useState } from 'react';
import { X } from 'lucide-react';
import { api } from '../api';

type Props = { onClose: () => void };

export function Settings({ onClose }: Props) {
  const [provider, setProvider] = useState<'openai' | 'anthropic' | 'ollama'>('ollama');
  const [model, setModel] = useState('llama3.1:8b');
  const [apiKey, setApiKey] = useState('');
  const [baseUrl, setBaseUrl] = useState('');
  const [msg, setMsg] = useState<string | null>(null);
  const [err, setErr] = useState<string | null>(null);

  const save = async () => {
    setErr(null); setMsg(null);
    try {
      await api.setAdvisor(provider, model, provider === 'ollama' ? undefined : apiKey, baseUrl || undefined);
      setMsg('Advisor configured.');
    } catch (e) {
      setErr(String(e));
    }
  };

  return (
    <div className="modal-bg" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-head">
          <div>Settings — AI Advisor</div>
          <button className="ghost icon" onClick={onClose}><X size={16} /></button>
        </div>

        <label className="field">
          <span>Provider</span>
          <select value={provider} onChange={(e) => {
            const v = e.target.value as 'openai' | 'anthropic' | 'ollama';
            setProvider(v);
            setModel(v === 'openai' ? 'gpt-4o-mini' : v === 'anthropic' ? 'claude-haiku-4-5' : 'llama3.1:8b');
            setBaseUrl('');
          }}>
            <option value="ollama">Ollama (local)</option>
            <option value="openai">OpenAI</option>
            <option value="anthropic">Anthropic</option>
          </select>
        </label>

        <label className="field">
          <span>Model</span>
          <input value={model} onChange={(e) => setModel(e.target.value)} />
        </label>

        {provider !== 'ollama' && (
          <label className="field">
            <span>API key</span>
            <input type="password" value={apiKey} onChange={(e) => setApiKey(e.target.value)} placeholder={provider === 'openai' ? 'sk-...' : 'sk-ant-...'} />
          </label>
        )}

        <label className="field">
          <span>Base URL (optional)</span>
          <input
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder={
              provider === 'ollama'
                ? 'http://localhost:11434'
                : provider === 'openai'
                  ? 'https://api.openai.com/v1'
                  : 'https://api.anthropic.com'
            }
          />
        </label>

        {msg && <div className="ok">{msg}</div>}
        {err && <div className="error">{err}</div>}

        <div className="modal-actions">
          <button className="primary" onClick={save}>Save</button>
          <button className="ghost" onClick={onClose}>Close</button>
        </div>

        <p className="muted" style={{ marginTop: 8 }}>
          Diskwise sends only directory metadata (path, size, file_count, top extensions, sample paths) — never file contents. Pick Ollama to keep all inference on-device.
        </p>
      </div>
    </div>
  );
}
