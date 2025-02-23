<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

class CreateUsersTable extends Migration
{
    public function up()
    {
        // First create the role enum type
        $this->execute("CREATE TYPE role AS ENUM ('admin', 'user', 'moderator')");
        
        // Then create the users table
        $this->createTable('users', [
            'id' => $this->primaryKey(),
            // ... other columns ...
            'role' => 'role',  // Now this will work since the type exists
            // ... remaining columns ...
        ]);
    }

    public function down()
    {
        $this->dropTable('users');
        // Don't forget to drop the type in down() method
        $this->execute('DROP TYPE role');
    }
} 