# Backup/Restore

The application's profile database can be exported into a backup file, which contains the following:

* Keys
* Charsets
* Custom key icons

## Backing Up Your Keys

1. Open **Settings** > **Backup/Restore** menu.
2. Click on the **Backup** button and save the file.

## Restoring Your Keys

Please note that restoring from a backup will **OVERRIDE** all of your profile database. Due to the possibility of a gap between the time of the backup and the restore, any modifications made after the backup file was created may be lost. Therefore, it's best practice to create a backup before performing a restore operation.

For restoring from a backup:

1. Open **Settings** > **Backup/Restore** menu.
2. Click on the **Restore** button.
3. Select your backup (`.kb`) file and confirm the restore.

<div class="warning">

> All backup files are signed with your master password. If your current master password does not match the backup's signature, the application will display a warning. You can still restore from a backup with a different signature, but the generated passwords will not be the same as the backup source.

</div>
