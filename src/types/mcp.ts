// MCP関連の型定義

export interface BacklogWorkspace {
  id: string;
  name: string;
  domain: string;
  apiKeyEncrypted: string;
  enabled: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface MCPServerStatus {
  running: boolean;
  version: string;
  connectedWorkspaces: string[];
  lastSync?: string;
}