import { useState } from 'react';
import { ChevronRight, ChevronDown, Folder, FolderOpen } from 'lucide-react';
import type { Node } from '../types';
import { formatBytes, formatCount } from '../format';

type Props = {
  root: Node;
  selectedPath: string | null;
  onSelect: (p: string) => void;
};

export function TreeView({ root, selectedPath, onSelect }: Props) {
  return (
    <div className="treeview">
      <Row node={root} depth={0} selectedPath={selectedPath} onSelect={onSelect} initialOpen />
    </div>
  );
}

function Row({
  node,
  depth,
  selectedPath,
  onSelect,
  initialOpen = false,
}: {
  node: Node;
  depth: number;
  selectedPath: string | null;
  onSelect: (p: string) => void;
  initialOpen?: boolean;
}) {
  const [open, setOpen] = useState(initialOpen);
  const hasKids = node.children.length > 0;
  const sel = node.path === selectedPath;
  return (
    <>
      <div
        className={'tree-row' + (sel ? ' selected' : '')}
        style={{ paddingLeft: 8 + depth * 14 }}
        onClick={() => onSelect(node.path)}
      >
        <span className="caret" onClick={(e) => { e.stopPropagation(); setOpen((v) => !v); }}>
          {hasKids ? open ? <ChevronDown size={12} /> : <ChevronRight size={12} /> : <span style={{ width: 12 }} />}
        </span>
        {open ? <FolderOpen size={14} /> : <Folder size={14} />}
        <span className="name" title={node.path}>{node.name}</span>
        {node.scaffold_id && <span className="badge">{node.scaffold_id}</span>}
        <span className="size">{formatBytes(node.size)}</span>
        <span className="count">{formatCount(node.file_count)}</span>
      </div>
      {open && hasKids && node.children.slice(0, 200).map((c) => (
        <Row key={c.path} node={c} depth={depth + 1} selectedPath={selectedPath} onSelect={onSelect} />
      ))}
    </>
  );
}
