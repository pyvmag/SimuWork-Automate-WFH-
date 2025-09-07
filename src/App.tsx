import { useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

const screenWidth = window.screen.width || 1920;
const screenHeight = window.screen.height || 1080;

function ConfigurationPanel() {
  const [activityLevel, setActivityLevel] = useState('Medium');
  // --- ADDED BACK ---
  const [mouseSpeed, setMouseSpeed] = useState(50);
  const [wpm, setWpm] = useState(40);
  // ---
  const [isRunning, setIsRunning] = useState(false);
  const [targetCoords, setTargetCoords] = useState({ x: 0.5, y: 0.5 });
  const [toastMessage, setToastMessage] = useState('');
  const targetAreaRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (toastMessage) {
      const timer = setTimeout(() => setToastMessage(''), 3000);
      return () => clearTimeout(timer);
    }
  }, [toastMessage]);

  const handleStart = async () => {
    setIsRunning(true);
    console.log('Sending start command to backend...');

    const absoluteCoords = {
      x: Math.round(targetCoords.x * screenWidth),
      y: Math.round(targetCoords.y * screenHeight),
    };

    // Pass mouseSpeed to the backend as well
    await invoke('start_simulation', { level: activityLevel, coords: absoluteCoords, mouseSpeed });
  };

  const handleStop = async () => {
    setIsRunning(false);
    console.log('Sending stop command to backend...');
    await invoke('stop_simulation');
  };

  const handleTargetSelect = (e: React.MouseEvent<HTMLDivElement>) => {
    if (targetAreaRef.current) {
      const rect = targetAreaRef.current.getBoundingClientRect();
      const xPercent = (e.clientX - rect.left) / rect.width;
      const yPercent = (e.clientY - rect.top) / rect.height;
      setTargetCoords({ x: xPercent, y: yPercent });
      const absoluteX = Math.round(xPercent * screenWidth);
      const absoluteY = Math.round(yPercent * screenHeight);
      setToastMessage(`Target set to screen coordinates: (${absoluteX}, ${absoluteY})`);
    }
  };

  const displayX = Math.round(targetCoords.x * screenWidth);
  const displayY = Math.round(targetCoords.y * screenHeight);

  return (
    <div className="container">
      <h1>SimuWork Control Panel</h1>

      <div className="row">
        <label htmlFor="activity-level">Activity Level:</label>
        <select id="activity-level" value={activityLevel} onChange={(e) => setActivityLevel(e.target.value)} disabled={isRunning}>
          <option value="Low">Low</option>
          <option value="Medium">Medium</option>
          <option value="High">High</option>
        </select>
      </div>

      {/* --- ADDED BACK --- */}
      <div className="row">
        <label htmlFor="mouse-speed">Mouse Speed: {mouseSpeed}</label>
        <input type="range" id="mouse-speed" min="1" max="100" value={mouseSpeed} onChange={(e) => setMouseSpeed(parseInt(e.target.value, 10))} disabled={isRunning} />
      </div>
      <div className="row">
        <label htmlFor="wpm">Typing Speed (WPM):</label>
        <input type="number" id="wpm" value={wpm} onChange={(e) => setWpm(parseInt(e.target.value, 10))} disabled={isRunning} />
      </div>
      {/* --- */}

      <div className="row">
        <label>Typing Target: ({displayX}, {displayY})</label>
      </div>
      <div 
        ref={targetAreaRef}
        className="target-area" 
        onClick={handleTargetSelect}
        title="Click inside this area to set the target for typing/clicking on your real screen"
      >
        <div className="target-area-content"> {/* NEW WRAPPER */}
          Click to set target
        </div>
      </div>

      <div className="row">
        <button onClick={handleStart} disabled={isRunning}>Start</button>
        <button id="stop-button" onClick={handleStop} disabled={!isRunning}>Stop</button>
      </div>

      <div className={`toast-notification ${toastMessage ? 'show' : ''}`}>
        {toastMessage}
      </div>
    </div>
  );
}

export default ConfigurationPanel;