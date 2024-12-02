<script lang="ts">
  import {invoke} from "@tauri-apps/api/core";

  interface UserInfo {
    name: string;
    age: number;
  }
  let name: string = "";
  let age: string = ""; // Input type `number` values are returned as strings
  let key: string = "";
  let value: string = "";
  let userData: UserInfo | null = null;
  let secureData: string | null = null;

  // Save user data
  async function saveUserData() {

    if (name && age) {
      await invoke("save_user_data", {
        user: { name, age: parseInt(age, 10) },
      });
      alert("User data saved!");
    } else {
      alert("Please fill in both name and age.");
    }
  }

  // Load user data
  async function loadUserData() {
    const data = await invoke<UserInfo | null>("get_user_data");
    userData = data;
  }

  // Save secure data
  async function saveSecureData() {
    if (key && value) {
      await invoke("save_secure_data", { key, value });
      alert("Secure data saved!");
    } else {
      alert("Please fill in both key and value.");
    }
  }

  // Load secure data
  async function loadSecureData() {
    if (key) {
      const data = await invoke<string | null>("load_secure_data", { key });
      secureData = data;
    } else {
      alert("Please provide a key.");
    }
  }
</script>

<div class="min-h-screen bg-gray-100 flex flex-col items-center p-6">
  <div class="w-full max-w-2xl bg-white shadow-md rounded-lg p-6">
    <h1 class="text-2xl font-bold text-gray-800 mb-4">Tauri Secure Demo</h1>

    <!-- User Data Section -->
    <section class="mb-8">
      <h2 class="text-xl font-semibold text-gray-700 mb-2">User Data</h2>
      <div class="space-y-4">
        <input
                type="text"
                placeholder="Name"
                bind:value={name}
                class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
        />
        <input
                type="number"
                placeholder="Age"
                bind:value={age}
                class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
        />
        <div class="flex space-x-2">
          <button
                  on:click={saveUserData}
                  class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md shadow-sm"
          >
            Save User Data
          </button>
          <button
                  on:click={loadUserData}
                  class="bg-gray-500 hover:bg-gray-600 text-white px-4 py-2 rounded-md shadow-sm"
          >
            Load User Data
          </button>
        </div>
      </div>
      {#if userData}
        <pre class="bg-gray-100 p-4 rounded-md mt-4">
          {JSON.stringify(userData, null, 2)}
        </pre>
      {/if}
    </section>

    <!-- Secure Storage Section -->
    <section>
      <h2 class="text-xl font-semibold text-gray-700 mb-2">Secure Storage</h2>
      <div class="space-y-4">
        <input
                type="text"
                placeholder="Key"
                bind:value={key}
                class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
        />
        <input
                type="text"
                placeholder="Value"
                bind:value={value}
                class="w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
        />
        <div class="flex space-x-2">
          <button
                  on:click={saveSecureData}
                  class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md shadow-sm"
          >
            Save Secure Data
          </button>
          <button
                  on:click={loadSecureData}
                  class="bg-gray-500 hover:bg-gray-600 text-white px-4 py-2 rounded-md shadow-sm"
          >
            Load Secure Data
          </button>
        </div>
      </div>
      {#if secureData}
        <pre class="bg-gray-100 p-4 rounded-md mt-4">
          {secureData || "No data found"}
        </pre>
      {/if}
    </section>
  </div>
</div>
