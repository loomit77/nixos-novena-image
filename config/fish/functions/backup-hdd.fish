function backup-hdd --description 'Novena backup: hdd'
    command bash "$HOME/nixos-novena-image/scripts/novena-backup.bash" --hdd $argv
    return $status
end
