<template>
  <div
    class="min-h-screen flex flex-col bg-gradient-to-b from-pink-100 via-pink-50 to-rose-100 font-[Nunito] noselect transition-all"
  >
    <Navbar />
    <main class="flex-grow px-4 py-8">
      <div class="max-w-4xl mx-auto space-y-6 animate-fade-in-up">
        <div class="flex flex-col items-center gap-2">
          <img
            src="/assets/logo.png"
            alt="Nekoi Gallery"
            class="h-24 w-24 animate-wiggle-slow drop-shadow-lg"
            draggable="false"
          />
          <h1 class="text-4xl font-extrabold text-pink-600 drop-shadow-pink">
            Image <span class="text-orange-400">Gallery</span>
          </h1>
        </div>
        <div
          v-if="!user || status === 'unauthenticated'"
          class="text-pink-700 text-lg font-medium backdrop-blur-sm bg-white/40 p-4 rounded-xl shadow-md text-center"
        >
          Please log in to view the gallery.
          <NuxtLink
            to="/login"
            class="inline-block mt-2 px-4 py-2 bg-pink-500 text-white font-semibold rounded-lg shadow-md hover:bg-pink-600 transition-colors duration-300"
          >
            Go to Login
          </NuxtLink>
        </div>
        <div
          v-else-if="status === 'loading'"
          class="text-pink-700 text-lg font-medium backdrop-blur-sm bg-white/40 p-4 rounded-xl shadow-md text-center"
        >
          Loading gallery...
        </div>
        <div v-else class="space-y-6">
          <div class="backdrop-blur-sm bg-white/40 p-6 rounded-xl shadow-md">
            <div class="flex flex-col sm:flex-row gap-4 justify-center mb-6">
              <div class="flex flex-col">
                <label for="type" class="text-pink-700 font-semibold mb-1">Content Type</label>
                <select
                  id="type"
                  v-model="selectedType"
                  class="px-4 py-2 bg-white/50 border border-pink-200 rounded-lg text-pink-700 focus:outline-none focus:ring-2 focus:ring-pink-500"
                >
                  <option value="sfw">SFW</option>
                  <option value="nsfw">NSFW</option>
                </select>
              </div>
              <div class="flex flex-col">
                <label for="category" class="text-pink-700 font-semibold mb-1">Category</label>
                <select
                  id="category"
                  v-model="selectedCategory"
                  class="px-4 py-2 bg-white/50 border border-pink-200 rounded-lg text-pink-700 focus:outline-none focus:ring-2 focus:ring-pink-500"
                >
                  <option v-for="category in availableCategories" :key="category" :value="category">
                    {{ category.charAt(0).toUpperCase() + category.slice(1) }}
                  </option>
                </select>
              </div>
              <button
                @click="fetchImage"
                class="px-6 py-2 bg-pink-500 text-white font-semibold rounded-lg shadow-md hover:bg-pink-600 transition-colors duration-300 mt-6 sm:mt-0"
                :disabled="isLoading"
              >
                {{ isLoading ? "Loading..." : "Load Image" }}
              </button>
            </div>
            <div
              v-if="error"
              class="text-pink-700 text-lg font-medium text-center mb-6"
            >
              {{ error.message || "Failed to load image. Try another category!" }}
            </div>
            <div v-if="image && Object.keys(image).length" class="flex flex-col items-center gap-4 mb-6">
              <h2 class="text-2xl font-bold text-pink-600">
                {{ selectedType.toUpperCase() }} - {{ selectedCategory.charAt(0).toUpperCase() + selectedCategory.slice(1) }}
              </h2>
              <div class="text-lg font-semibold text-pink-700">ID: {{ image.id }}</div>
              <img
                :src="image.url"
                :alt="`Image ${image.id}`"
                class="max-w-full max-h-[600px] object-contain rounded-md"
                draggable="false"
                loading="lazy"
              />
            </div>
            <div
              v-else-if="!isLoading && !error"
              class="text-pink-700 text-lg font-medium text-center mb-6"
            >
              No image found for this category.
            </div>
            <div v-if="rateLimits && Object.keys(rateLimits).length" class="mt-6">
              <h3 class="text-xl font-bold text-pink-600 mb-4">Rate Limit Information</h3>
              <div class="overflow-x-auto">
                <table class="w-full text-left text-pink-700 table-auto bg-white/50 border border-pink-200/50 rounded-lg">
                  <thead>
                    <tr class="border-b border-pink-200/50 bg-pink-100/50">
                      <th class="py-4 px-6 font-semibold">Header</th>
                      <th class="py-4 px-6 font-semibold">Value</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr class="border-b border-pink-200/50 hover:bg-white/70">
                      <td class="py-4 px-6">X-RateLimit-Limit</td>
                      <td class="py-4 px-6">{{ rateLimits['x-ratelimit-limit'] || 'N/A' }}</td>
                    </tr>
                    <tr class="border-b border-pink-200/50 hover:bg-white/70">
                      <td class="py-4 px-6">X-RateLimit-Remaining</td>
                      <td class="py-4 px-6">{{ rateLimits['x-ratelimit-remaining'] || 'N/A' }}</td>
                    </tr>
                    <tr class="hover:bg-white/70">
                      <td class="py-4 px-6">X-RateLimit-Reset</td>
                      <td class="py-4 px-6">{{ formatSecondsVerbose(rateLimits['x-ratelimit-reset']) || 'N/A' }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
const { user, status, token } = useAuth();
const toast = useToast();
const isLoading = ref(false);
const error = ref(null);
const image = ref({});
const rateLimits = ref({});
const selectedType = ref("sfw");
const selectedCategory = ref("neko");
const config = useAppConfig();

const categories = {
  sfw: config.SFW_CATEGORIES,
  nsfw: config.NSFW_CATEGORIES,
};

const availableCategories = computed(() => categories[selectedType.value] || []);

function formatSecondsVerbose(seconds) {
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const parts = [];
  if (hrs) parts.push(`${hrs} hour${hrs !== 1 ? 's' : ''}`);
  if (mins) parts.push(`${mins} minute${mins !== 1 ? 's' : ''}`);
  if (secs || parts.length === 0) parts.push(`${secs} second${secs !== 1 ? 's' : ''}`);

  return parts.join(', ');
}

watch(selectedType, (newType) => {
  if (!categories[newType].includes(selectedCategory.value)) {
    selectedCategory.value = categories[newType][0] || "neko";
  }
});

const fetchImage = async () => {
  if (!user.value || !token.value) {
    error.value = new Error("You must be logged in to view images.");
    return;
  }

  isLoading.value = true;
  error.value = null;
  image.value = {};
  rateLimits.value = {};

  try {
    const response = await $fetch(`/api/v1/${selectedType.value}/${selectedCategory.value}`, {
      method: "GET",
      headers: {
        Authorization: user.value.api_key,
        "Content-Type": "application/json",
      },
      onResponse({ response: res }) {

        rateLimits.value = {
          'x-ratelimit-limit': res.headers.get('x-ratelimit-limit'),
          'x-ratelimit-remaining': res.headers.get('x-ratelimit-remaining'),
          'x-ratelimit-reset': res.headers.get('x-ratelimit-reset'),
        };
      },
    });
    image.value = response || {};
    if (!Object.keys(image.value).length) {
      toast.add({
        description: `No image found for ${selectedType.value}/${selectedCategory.value}.`,
        color: "warning",
      });
    }
  } catch (err) {
    error.value = new Error(err.message || "Failed to fetch image");
    toast.add({
      description: `Error: ${err.message || "Failed to load image"}`,
      color: "error",
    });
  } finally {
    isLoading.value = false;
  }
};

onMounted(async () => {
  if (user.value && status.value === "authenticated") {
    await fetchImage();
  }
});

watch([selectedType, selectedCategory], async () => {
  if (status.value === "authenticated") {
    await fetchImage();
  }
});

useHead({
  title: computed(() => `Gallery — Nekoi`),
  meta: [
    {
      property: "og:title",
      content: computed(() => `Gallery — Nekoi`),
    },
    { property: "og:site_name", content: "2rkf" },
    {
      property: "og:description",
      content: "Explore a collection of high-quality anime-style images on Nekoi.",
    },
    { property: "og:image", content: "/nekoi.png" },
    { property: "og:image:type", content: "image/png" },
    { property: "og:image:width", content: "1200" },
    { property: "og:image:height", content: "630" },
    { name: "theme-color", content: "#ffd8b1" },
    { "http-equiv": "x-ua-compatible", content: "IE=edge" },
    { name: "viewport", content: "width=device-width, initial-scale=1.0" },
  ],
});
</script>

<style scoped>
.drop-shadow-pink {
  filter: drop-shadow(0 4px 6px rgba(244, 114, 182, 0.3));
}

/* Table row hover effect */
tbody tr:hover {
  background-color: rgba(255, 255, 255, 0.7);
}
</style>
