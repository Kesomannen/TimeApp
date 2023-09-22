export type Project = {
    display_name: string
    open: boolean
    time: {
        secs: number
        nanos: number
    }
}

export type UpdatePayload = {
    project_names: string[]
    projects: Project[]
}

export type Engine = 'Unity' | 'Godot';
export type Theme = 'Light' | 'Dark';
