<script lang="ts">
    import { Input } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { isDateOnlyEqual } from "../../data/util/time";

    interface Props {
        selectedTime?: DateTime | null;
        disabled?: boolean;
        min?: DateTime | undefined;
        max?: DateTime | undefined;
        class?: string;
        onupdate: () => void;
    }

    let {
        selectedTime = $bindable(null),
        disabled = false,
        min = undefined,
        max = undefined,
        class: className,
        onupdate,
    }: Props = $props();

    async function onHourChange(e: Event, value: "hour" | "minute") {
        e.preventDefault();
        const t = e.target as HTMLInputElement;
        let v = parseInt(t.value);

        if (isNaN(v)) {
            v = 0;
        }

        const minToday =
            min && selectedTime && isDateOnlyEqual(min, selectedTime)
                ? min
                : undefined;
        const maxToday =
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

        onupdate();
    }
</script>

<div class={`flex items-start  ${className ?? ""}`}>
    <div class="flex items-center gap-4">
        <Input
            size="lg"
            type="number"
            placeholder="hh"
            value={selectedTime?.toFormat("HH")}
            oninput={(e) => onHourChange(e, "hour")}
            {disabled}
        />
        <p class="font-display text-2xl font-bold">:</p>
        <Input
            size="lg"
            type="number"
            placeholder="mm"
            value={selectedTime?.toFormat("mm")}
            oninput={(e) => onHourChange(e, "minute")}
            {disabled}
        />
    </div>
</div>
