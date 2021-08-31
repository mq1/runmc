<script setup lang="ts">
import { fetchGameVersions } from '~/logic/launchermeta'
import type { GameVersion } from '~/logic/launchermeta'
import { newInstance } from '~/logic/instances'

const { t } = useI18n()
const router = useRouter()

const name = ref('')
const version: Ref<GameVersion | undefined> = ref()
const snapshotsEnabled = ref(false)
const installing = ref(false)

const versions: Ref<GameVersion[]> = ref([])
const updateVersions = async() =>
  versions.value = await fetchGameVersions()

const create = () => {
  installing.value = true

  newInstance(name.value, version.value!)
    .then(() => router.push('/instances'))
}

onMounted(updateVersions)
</script>

<template>
  <h1 class="text-3xl text-center">
    {{ t('instances.new') }}
  </h1>

  <div class="flex flex-col gap-y-4 min-w-xs">
    <label class="flex flex-col">
      <span>{{ t('name') }}</span>
      <input v-model="name" type="text" />
    </label>
  </div>

  <div class="box border flex flex-col gap-y-4 w-full overflow-y-auto">
    <h2 class="text-2xl text-center">
      {{ t('instances.availableversions') }}
    </h2>

    <div class="grid grid-cols-2 gap-x-8 w-full overflow-y-auto">
      <div class="flex flex-col gap-y-4 overflow-y-auto w-full">
        <div v-if="versions" class="overflow-y-auto h-full w-full flex flex-col divide-y">
          <div v-for="gv in versions.filter(gv => gv.type === 'release' || snapshotsEnabled)" :key="gv.id" class="flex justify-start items-center gap-x-2 py-1">
            <input :id="gv.id" v-model="version" type="radio" :value="gv" />
            <label :for="gv.id">
              {{ gv.type }}
              {{ gv.id }}
            </label>
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-y-4">
        <h2 class="text-xl">
          {{ t('filters') }}
        </h2>
        <div class="flex items-center gap-x-2">
          <input v-model="snapshotsEnabled" type="checkbox" />
          <label>{{ t('instances.showsnapshots') }}</label>
        </div>
      </div>
    </div>
  </div>

  <div class="w-full flex justify-end gap-x-2">
    <button v-if="!installing" class="w-auto bg-green-500 text-white flex gap-x-2" @click="create">
      <carbon-new-tab />
      <span>
        {{ t('instances.create') }}
      </span>
    </button>
    <div v-if="installing" class="flex items-center gap-x-2">
      <carbon-restart class="animate-spin" />
      <span>
        {{ t('installing') }}
      </span>
    </div>
  </div>
</template>
