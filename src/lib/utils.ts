export function format_time(total_secs: number) {
    const hours = Math.floor(total_secs / 3600)
    const minutes = Math.floor((total_secs - (hours * 3600)) / 60)
    const secs = total_secs - (hours * 3600) - (minutes * 60)

    const hours_str = hours < 10 ? `0${hours}` : `${hours}`
    const minutes_str = minutes < 10 ? `0${minutes}` : `${minutes}`
    const secs_str = secs < 10 ? `0${secs}` : `${secs}`

    return `${hours_str}:${minutes_str}:${secs_str}`;
}
