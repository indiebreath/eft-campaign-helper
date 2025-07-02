<script setup>
import { invoke } from "@tauri-apps/api/core";

invoke("hello_world");
</script>
