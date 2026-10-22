function backup-usb --description 'Novena backup: usb'
    command bash "$HOME/nixos-novena-image/scripts/novena-backup.bash" --usb $argv
    return $status
end
