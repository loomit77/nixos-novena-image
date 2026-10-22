function backup-all --description 'Novena backup: all'
    command bash "$HOME/nixos-novena-image/scripts/novena-backup.bash" --all $argv
    return $status
end
