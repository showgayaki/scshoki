interface CheckboxProps {
    checked: boolean;
    onChange: (checked: boolean) => void;
    label: string;
}

export default function Checkbox({ checked, onChange, label }: CheckboxProps) {
    return (
        <label className="block text-sm font-medium text-gray-700">
            <input
                type="checkbox"
                checked={checked}
                onChange={() => onChange(!checked)}
                className="mr-2"
            />
            {label}
        </label>
    );
}
