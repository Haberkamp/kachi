import { useEffect, useState, useCallback, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface KeyEvent {
  key: string;
  event_type: "press" | "release";
}

interface DisplayKey {
  id: number;
  key: string;
  timestamp: number;
}

const MODIFIERS = ["⌘", "Ctrl", "Alt", "Shift", "AltGr"];
const FADE_DELAY = 2000;

function App() {
  const [activeModifiers, setActiveModifiers] = useState<Set<string>>(
    new Set()
  );
  const [recentKeys, setRecentKeys] = useState<DisplayKey[]>([]);
  const keyIdRef = useRef(0);

  const addKey = useCallback((key: string) => {
    const newKey: DisplayKey = {
      id: keyIdRef.current++,
      key,
      timestamp: Date.now(),
    };
    setRecentKeys((prev) => [...prev.slice(-4), newKey]);
  }, []);

  useEffect(() => {
    const unlisten = listen<KeyEvent>("key-event", (event) => {
      const { key, event_type } = event.payload;

      if (MODIFIERS.includes(key)) {
        setActiveModifiers((prev) => {
          const next = new Set(prev);
          if (event_type === "press") {
            next.add(key);
          } else {
            next.delete(key);
          }
          return next;
        });
      } else if (event_type === "press") {
        addKey(key);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [addKey]);

  // Clean up old keys
  useEffect(() => {
    const interval = setInterval(() => {
      const now = Date.now();
      setRecentKeys((prev) =>
        prev.filter((k) => now - k.timestamp < FADE_DELAY)
      );
    }, 100);

    return () => clearInterval(interval);
  }, []);

  const modifierOrder = ["Ctrl", "Alt", "Shift", "⌘"];
  const sortedModifiers = modifierOrder.filter((m) => activeModifiers.has(m));

  const hasContent = sortedModifiers.length > 0 || recentKeys.length > 0;

  return (
    <div className="container" data-tauri-drag-region>
      {hasContent && (
        <div className="key-display">
          {sortedModifiers.map((mod) => (
            <span key={mod} className="key modifier">
              {mod}
            </span>
          ))}
          {sortedModifiers.length > 0 && recentKeys.length > 0 && (
            <span className="plus">+</span>
          )}
          {recentKeys.map((k, index) => (
            <span
              key={k.id}
              className="key"
              style={{
                animationDelay: `${index * 30}ms`,
                opacity: Math.max(
                  0,
                  1 - (Date.now() - k.timestamp) / FADE_DELAY
                ),
              }}
            >
              {k.key}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
