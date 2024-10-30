import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";

// Recording State
export interface RecordingState {
  isRecording: boolean;
  currentFile: string | null;
  lastRecording: string | null;
}

export const recordingState = writable<RecordingState>({
  isRecording: false,
  currentFile: null,
  lastRecording: null,
});

// Connection State
export interface ConnectionState {
  connected: boolean;
  lastConnected: Date | null;
  lastDisconnected: Date | null;
}

export const connectionState = writable<ConnectionState>({
  connected: false,
  lastConnected: null,
  lastDisconnected: null,
});

// Listen for recording status updates
listen("recording-status", (event: any) => {
  const { isRecording, currentFile } = event.payload;

  recordingState.update((state) => ({
    isRecording,
    currentFile: isRecording ? currentFile : null,
    lastRecording:
      !isRecording && currentFile ? currentFile : state.lastRecording,
  }));
});

// Listen for OBS connection status
listen("obs-status", (event: any) => {
  const { connected } = event.payload;
  console.log(`connected updated ${connected}`);
  connectionState.update((state) => ({
    connected,
    lastConnected: connected ? new Date() : state.lastConnected,
    lastDisconnected: !connected ? new Date() : state.lastDisconnected,
  }));
});
