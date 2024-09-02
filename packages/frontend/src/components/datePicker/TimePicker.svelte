<script lang="ts">
    import { Input } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { createEventDispatcher } from "svelte";
    import { isDateOnlyEqual } from "../../data/util/time";

    export let selectedTime: DateTime | null = null;
    export let disabled = false;
    export let min: DateTime | undefined = undefined;
    export let max: DateTime | undefined = undefined;
    const dispatch = createEventDispatcher<{ update: undefined }>();

    $: onHourChange = async (e: Event, value: "hour" | "minute") => {
        e.preventDefault();
        const t = e.target as HTMLInputElement;
        let v = parseInt(t.value);

        if (isNaN(v)) {
            v = 0;
        }

        let minToday =
            min && selectedTime && isDateOnlyEqual(min, selectedTime)
                ? min
                : undefined;
        let maxToday =
            max && selectedTime && isDateOnlyEqual(max, selectedTime)
                ? max
                : undefined;

        if (value === "hour") {
            v = Math.min(
                maxToday?.hour ?? 23,
                Math.max(minToday?.hour ?? 0, v)
            );
        }
        if (value === "minute") {
            v = Math.min(59, Math.max(0, v));
        }

        if (!selectedTime) {
            const newDate = DateTime.now().set({
                [value]: v,
            });
            selectedTime = newDate;
        } else {
            const newDate = selectedTime.set({ [value]: v });
            selectedTime = newDate;
        }

        dispatch("update");
    };
</script>

<div class={`flex items-start  ${$$props.class}`}>
    <div class="flex items-center gap-4">
        <Input
            size="lg"
            type="number"
            placeholder="hh"
            value={selectedTime?.toFormat("HH")}
            on:input={(e) => onHourChange(e, "hour")}
            {disabled}
        />
        <p class="font-display text-2xl font-bold">:</p>
        <Input
            size="lg"
            type="number"
            placeholder="mm"
            value={selectedTime?.toFormat("mm")}
            on:input={(e) => onHourChange(e, "minute")}
            {disabled}
        />
    </div>
</div>
