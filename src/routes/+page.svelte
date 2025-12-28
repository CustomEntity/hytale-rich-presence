<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";

  // State
  let isConnected = $state(false);
  let isLoading = $state(false);
  let selectedActivity = $state("exploring");
  let customActivity = $state("");
  let serverName = $state("");
  let showElapsed = $state(true);
  let statusMessage = $state("");
  let isUpdating = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  // Party state
  let onServer = $state(false);
  let partySize = $state(1);
  let partyMax = $state(10);
  let joinServerUrl = $state("");
  let showOrbisButton = $state(true);

  // Activity options
  const activities = [
    { id: "exploring", label: "Exploring the Alterverse", icon: "🌍" },
    { id: "building", label: "Building in Creative Mode", icon: "🏗️" },
    { id: "pvp", label: "Fighting in PvP Arena", icon: "⚔️" },
    { id: "dungeon", label: "Raiding a Dungeon", icon: "🏰" },
    { id: "taming", label: "Taming Creatures", icon: "🐉" },
    { id: "crafting", label: "Crafting Items", icon: "🔨" },
    { id: "adventure", label: "Adventuring with Friends", icon: "👥" },
    { id: "custom", label: "Custom...", icon: "✏️" },
  ];

  async function toggleConnection() {
    isLoading = true;
    statusMessage = "";

    try {
      if (isConnected) {
        await invoke("disconnect_discord");
        isConnected = false;
        statusMessage = "Disconnected from Discord";
      } else {
        const result = await invoke("connect_discord");
        isConnected = true;
        // Wait a moment for Discord to be ready
        await new Promise((resolve) => setTimeout(resolve, 100));
        await updateActivity();
        statusMessage = "Connected! Your presence is now visible";
      }
    } catch (error: any) {
      const errorMsg = error?.toString() || "Unknown error";
      if (errorMsg.includes("Failed to connect")) {
        statusMessage = "Error: Make sure Discord is running";
      } else {
        statusMessage = `Error: ${errorMsg}`;
      }
      isConnected = false;
    }

    isLoading = false;
  }

  async function updateActivity() {
    if (!isConnected || isLoading || isUpdating) return;

    isUpdating = true;
    try {
      await invoke("update_activity", {
        config: {
          activity_type: selectedActivity,
          custom_activity:
            selectedActivity === "custom" ? customActivity : null,
          server_name: serverName || null,
          show_elapsed: showElapsed,
          on_server: onServer && serverName.trim() !== "",
          party_size: partySize,
          party_max: partyMax,
          join_server_url: joinServerUrl || null,
          show_orbis_button: showOrbisButton,
        },
      });
    } catch (error: any) {
      const errorMsg = error?.toString() || "Unknown error";
      if (!errorMsg.includes("Not connected")) {
        statusMessage = `Failed to update: ${errorMsg}`;
      }
    }
    isUpdating = false;
  }

  // Debounced update function
  function debouncedUpdate() {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      if (isConnected && !isLoading) {
        updateActivity();
      }
    }, 300);
  }

  // Update activity when settings change (only if connected)
  $effect(() => {
    if (isConnected && !isLoading) {
      // Trigger when these change
      selectedActivity;
      customActivity;
      serverName;
      showElapsed;
      onServer;
      partySize;
      partyMax;
      joinServerUrl;
      showOrbisButton;
      debouncedUpdate();
    }
  });

  async function openOrbis() {
    await openUrl("https://orbis.place");
  }
</script>

<main class="app">
  <!-- Header -->
  <header class="header">
    <div class="logo-section">
      <img src="/hytale.webp" alt="Hytale Logo" class="hytale-logo" />
      <div class="title-section">
        <h1>Hytale Rich Presence</h1>
        <p class="subtitle">Show off your Hytale hype on Discord</p>
      </div>
    </div>
    <div class="status-indicator" class:connected={isConnected}>
      <span class="status-dot"></span>
      <span class="status-text"
        >{isConnected ? "Connected" : "Disconnected"}</span
      >
    </div>
  </header>

  <!-- Main Content -->
  <div class="content">
    <!-- Activity Section -->
    <section class="section">
      <label class="section-label">What are you doing?</label>
      <div class="activity-grid">
        {#each activities as activity}
          <button
            class="activity-card"
            class:selected={selectedActivity === activity.id}
            onclick={() => (selectedActivity = activity.id)}
          >
            <span class="activity-icon">{activity.icon}</span>
            <span class="activity-label">{activity.label}</span>
          </button>
        {/each}
      </div>
      {#if selectedActivity === "custom"}
        <input
          type="text"
          class="server-input custom-activity-input"
          placeholder="Enter your custom activity..."
          bind:value={customActivity}
          maxlength="100"
        />
      {/if}
    </section>

    <!-- Party Section -->
    <section class="section">
      <label class="checkbox-wrapper">
        <input type="checkbox" bind:checked={onServer} />
        <span class="checkmark"></span>
        <span class="checkbox-label">On a Server (show party)</span>
      </label>

      {#if onServer}
        <div class="server-name-field">
          <label class="section-label" for="server-input">Server Name</label>
          <input
            id="server-input"
            type="text"
            class="server-input"
            placeholder="e.g., Orbis Community Server"
            bind:value={serverName}
            maxlength="50"
          />
        </div>

        <div class="party-inputs">
          <div class="party-field">
            <label for="party-size">Players</label>
            <input
              id="party-size"
              type="number"
              min="1"
              max={partyMax}
              bind:value={partySize}
              class="number-input"
            />
          </div>
          <span class="party-separator">of</span>
          <div class="party-field">
            <label for="party-max">Max</label>
            <input
              id="party-max"
              type="number"
              min="1"
              max="100"
              bind:value={partyMax}
              class="number-input"
            />
          </div>
        </div>

        <div class="join-url-section">
          <label class="section-label" for="join-url"
            >Join Server Button URL (optional)</label
          >
          <input
            id="join-url"
            type="url"
            class="server-input"
            placeholder="https://your-server-invite-link.com"
            bind:value={joinServerUrl}
          />
        </div>

        <label class="checkbox-wrapper orbis-toggle">
          <input type="checkbox" bind:checked={showOrbisButton} />
          <span class="checkmark"></span>
          <span class="checkbox-label"
            >Show "Discover more on Orbis" button</span
          >
        </label>
      {/if}
    </section>

    <!-- Connect Button -->
    <button
      class="connect-btn"
      class:connected={isConnected}
      class:loading={isLoading}
      onclick={toggleConnection}
      disabled={isLoading}
    >
      {#if isLoading}
        <span class="spinner"></span>
        <span>Connecting...</span>
      {:else if isConnected}
        <svg class="discord-icon" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"
          />
        </svg>
        <span>Disconnect</span>
      {:else}
        <svg class="discord-icon" viewBox="0 0 24 24" fill="currentColor">
          <path
            d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"
          />
        </svg>
        <span>Connect to Discord</span>
      {/if}
    </button>

    <!-- Status Message -->
    {#if statusMessage}
      <p class="status-message" class:error={statusMessage.includes("Error")}>
        {statusMessage}
      </p>
    {/if}
  </div>

  <!-- Footer -->
  <footer class="footer">
    <button class="orbis-link" onclick={openOrbis}>
      <span class="orbis-text">Powered by</span>
      <img src="/orbis-logo.png" alt="Orbis" class="orbis-logo" />
      <span class="orbis-arrow">→</span>
    </button>
  </footer>
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: "Outfit", sans-serif;
    background-color: #0f172a;
    background-image: radial-gradient(
        at 0% 0%,
        rgba(45, 212, 191, 0.15) 0px,
        transparent 50%
      ),
      radial-gradient(at 100% 0%, rgba(245, 158, 11, 0.1) 0px, transparent 50%);
    color: #e2e8f0;
    min-height: 100vh;
    overflow-x: hidden;
  }

  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    padding: 1.5rem;
    max-width: 800px;
    margin: 0 auto;
  }

  /* Header */
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid #334155;
  }

  .logo-section {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .hytale-logo {
    width: 64px;
    height: 64px;
    object-fit: contain;
    filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.3));
    transition: transform 0.3s ease;
  }

  .hytale-logo:hover {
    transform: scale(1.05) rotate(5deg);
  }

  .title-section h1 {
    font-family: "Cinzel", serif;
    font-size: 1.5rem;
    font-weight: 700;
    color: #f1f5f9;
    letter-spacing: 0.02em;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  }

  .subtitle {
    font-size: 0.875rem;
    color: #94a3b8;
    font-weight: 500;
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.5rem 1rem;
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 6px;
    box-shadow: 0 2px 0 #0f172a;
  }

  .status-dot {
    width: 10px;
    height: 10px;
    border-radius: 2px; /* Square dot */
    background: #ef4444;
    box-shadow: 0 0 8px rgba(239, 68, 68, 0.4);
    transition: all 0.3s ease;
    transform: rotate(45deg); /* Diamond shape */
  }

  .status-indicator.connected .status-dot {
    background: #22c55e;
    box-shadow: 0 0 8px rgba(34, 197, 94, 0.4);
  }

  .status-text {
    font-size: 0.75rem;
    font-weight: 700;
    color: #cbd5e1;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* Content */
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .section-label {
    font-family: "Cinzel", serif;
    font-size: 0.9375rem;
    font-weight: 700;
    color: #94a3b8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding-left: 0.5rem;
    border-left: 3px solid #2dd4bf;
    line-height: 1;
  }

  /* Activity Grid */
  .activity-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
  }

  .activity-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.1s ease;
    text-align: left;
    color: #cbd5e1;
    position: relative;
    top: 0;
    box-shadow: 0 4px 0 #0f172a;
  }

  .activity-card:hover {
    background: #334155;
    border-color: #475569;
    transform: translateY(-2px);
    box-shadow: 0 6px 0 #0f172a;
  }

  .activity-card:active {
    transform: translateY(2px);
    box-shadow: 0 2px 0 #0f172a;
  }

  .activity-card.selected {
    background: #134e4a;
    border-color: #2dd4bf;
    color: #f0fdfa;
    box-shadow: 0 4px 0 #115e59;
  }

  .activity-card.selected:hover {
    background: #115e59;
  }

  .activity-icon {
    font-size: 1.5rem;
    filter: drop-shadow(0 2px 2px rgba(0, 0, 0, 0.3));
  }

  .activity-label {
    font-size: 0.875rem;
    font-weight: 600;
    line-height: 1.2;
  }

  /* Server Input */
  .server-input {
    padding: 0.875rem 1rem;
    background: #0f172a;
    border: 2px solid #334155;
    border-radius: 8px;
    color: #f1f5f9;
    font-size: 1rem;
    font-family: "Outfit", sans-serif;
    transition: all 0.2s ease;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .server-input::placeholder {
    color: #475569;
  }

  .server-input:focus {
    outline: none;
    border-color: #f59e0b; /* Gold focus */
    box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.15);
  }

  /* Checkbox */
  .options-section {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 1.5rem;
    padding: 1rem;
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 8px;
  }

  .checkbox-wrapper {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    user-select: none;
  }

  .checkbox-wrapper input {
    display: none;
  }

  .checkmark {
    width: 22px;
    height: 22px;
    background: #0f172a;
    border: 2px solid #475569;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .checkbox-wrapper:hover .checkmark {
    border-color: #64748b;
  }

  .checkbox-wrapper input:checked + .checkmark {
    background: #2dd4bf;
    border-color: #14b8a6;
    box-shadow: none;
  }

  .checkbox-wrapper input:checked + .checkmark::after {
    content: "✓";
    color: #0f172a;
    font-size: 0.875rem;
    font-weight: 800;
  }

  .checkbox-label {
    font-size: 0.9375rem;
    font-weight: 500;
    color: #cbd5e1;
  }

  /* Party Inputs */
  .party-inputs {
    display: flex;
    align-items: flex-end;
    gap: 1rem;
    margin-top: 0.5rem;
    padding: 1rem;
    background: #1e293b;
    border: 2px solid #334155;
    border-radius: 8px;
    animation: slideDown 0.2s ease;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .party-field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .party-field label {
    font-family: "Cinzel", serif;
    font-size: 0.75rem;
    font-weight: 700;
    color: #94a3b8;
    text-transform: uppercase;
  }

  .number-input {
    width: 80px;
    padding: 0.625rem;
    background: #0f172a;
    border: 2px solid #334155;
    border-radius: 6px;
    color: #f1f5f9;
    font-size: 1.125rem;
    font-family: "Outfit", sans-serif;
    font-weight: 600;
    text-align: center;
    box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .number-input:focus {
    outline: none;
    border-color: #f59e0b;
  }

  .party-separator {
    font-family: "Cinzel", serif;
    font-size: 1rem;
    font-weight: 700;
    color: #64748b;
    padding-bottom: 0.75rem;
  }

  .join-url-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.75rem;
    animation: slideDown 0.2s ease;
  }

  .orbis-toggle {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: rgba(245, 158, 11, 0.1);
    border: 1px solid rgba(245, 158, 11, 0.2);
    border-radius: 6px;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 3px solid rgba(0, 0, 0, 0.1);
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  /* Status Message */
  .status-message {
    text-align: center;
    font-size: 0.875rem;
    font-weight: 600;
    color: #22c55e;
    padding: 0.75rem;
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.2);
    border-radius: 6px;
    animation: slideDown 0.3s ease;
  }

  .status-message.error {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.2);
  }

  /* Footer */
  .footer {
    margin-top: auto;
    padding-top: 2rem;
    display: flex;
    justify-content: center;
  }

  .orbis-link {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.75rem 1.25rem;
    background: transparent;
    border: 1px solid #334155;
    border-radius: 99px;
    cursor: pointer;
    transition: all 0.2s ease;
    font-family: inherit;
    color: #94a3b8;
  }

  .orbis-link:hover {
    border-color: #109eb1;
    color: #c7f4fa;
    background: rgba(16, 158, 177, 0.1);
  }

  .orbis-text {
    font-size: 0.75rem;
    font-weight: 500;
  }

  .orbis-logo {
    height: 20px;
    width: auto;
    object-fit: contain;
  }

  .orbis-arrow {
    font-size: 1rem;
    transition: transform 0.2s ease;
  }

  .orbis-link:hover .orbis-arrow {
    transform: translateX(3px);
  }

  /* Server Name Field */
  .server-name-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 0.5rem;
    animation: slideDown 0.2s ease;
  }

  .server-hint {
    font-size: 0.75rem;
    color: #64748b;
    font-style: italic;
  }

  /* Discord Icon */
  .discord-icon {
    width: 24px;
    height: 24px;
    flex-shrink: 0;
  }

  /* Connect Button - Discord Style */
  .connect-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    background: #5865f2;
    border: none;
    border-radius: 8px;
    color: #ffffff;
    font-size: 1rem;
    font-weight: 600;
    font-family: "Outfit", sans-serif;
    cursor: pointer;
    transition: all 0.15s ease;
    margin-top: 1rem;
    box-shadow: 0 4px 14px rgba(88, 101, 242, 0.4);
  }

  .connect-btn:hover:not(:disabled) {
    background: #4752c4;
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(88, 101, 242, 0.5);
  }

  .connect-btn:active:not(:disabled) {
    transform: translateY(1px);
    box-shadow: 0 2px 8px rgba(88, 101, 242, 0.4);
  }

  .connect-btn.connected {
    background: #ed4245;
    box-shadow: 0 4px 14px rgba(237, 66, 69, 0.4);
  }

  .connect-btn.connected:hover:not(:disabled) {
    background: #c93b3e;
    box-shadow: 0 6px 20px rgba(237, 66, 69, 0.5);
  }

  .connect-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .connect-btn.loading {
    background: #4752c4;
  }
</style>
