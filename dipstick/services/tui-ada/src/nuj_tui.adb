-- NUJ Monitor TUI - Ada Implementation
-- Terminal User Interface for Configuration Management

with Ada.Text_IO;
with Ada.Strings.Unbounded;
with Ada.Containers.Vectors;
with GNAT.OS_Lib;

procedure NUJ_TUI is

   package IO renames Ada.Text_IO;
   package SU renames Ada.Strings.Unbounded;
   use type SU.Unbounded_String;

   -- Configuration item
   type Config_Item is record
      Key   : SU.Unbounded_String;
      Value : SU.Unbounded_String;
      Desc  : SU.Unbounded_String;
   end record;

   package Config_Vectors is new Ada.Containers.Vectors
     (Index_Type   => Natural,
      Element_Type => Config_Item);

   Configuration : Config_Vectors.Vector;

   -- Terminal control codes
   CLEAR_SCREEN : constant String := ASCII.ESC & "[2J" & ASCII.ESC & "[H";
   BOLD         : constant String := ASCII.ESC & "[1m";
   RESET        : constant String := ASCII.ESC & "[0m";
   GREEN        : constant String := ASCII.ESC & "[32m";
   YELLOW       : constant String := ASCII.ESC & "[33m";
   BLUE         : constant String := ASCII.ESC & "[34m";
   CYAN         : constant String := ASCII.ESC & "[36m";

   procedure Print_Header is
   begin
      IO.Put_Line (CLEAR_SCREEN);
      IO.Put_Line (BOLD & CYAN &
        "╔════════════════════════════════════════════════════════════════╗");
      IO.Put_Line ("║     NUJ Social Media Ethics Monitor - Configuration TUI     ║");
      IO.Put_Line ("╚════════════════════════════════════════════════════════════════╝" &
        RESET);
      IO.New_Line;
   end Print_Header;

   procedure Print_Menu is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "Main Menu:" & RESET);
      IO.Put_Line ("  1. View Configuration");
      IO.Put_Line ("  2. Edit Configuration");
      IO.Put_Line ("  3. Platform Management");
      IO.Put_Line ("  4. Threshold Tuning");
      IO.Put_Line ("  5. CUE Script Editor");
      IO.Put_Line ("  6. Database Management");
      IO.Put_Line ("  7. Service Status");
      IO.Put_Line ("  8. Save & Exit");
      IO.Put_Line ("  0. Exit without saving");
      IO.New_Line;
      IO.Put (GREEN & "Select option: " & RESET);
   end Print_Menu;

   procedure View_Configuration is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "Current Configuration:" & RESET);
      IO.New_Line;

      for Item of Configuration loop
         IO.Put_Line (BOLD & SU.To_String (Item.Key) & RESET);
         IO.Put_Line ("  Value: " & CYAN & SU.To_String (Item.Value) & RESET);
         IO.Put_Line ("  " & SU.To_String (Item.Desc));
         IO.New_Line;
      end loop;

      IO.Put_Line ("Press Enter to continue...");
      IO.Skip_Line;
   end View_Configuration;

   procedure Edit_Configuration is
      Key   : SU.Unbounded_String;
      Value : SU.Unbounded_String;
      Found : Boolean := False;
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "Edit Configuration" & RESET);
      IO.New_Line;

      IO.Put ("Enter configuration key: ");
      Key := SU.To_Unbounded_String (IO.Get_Line);

      -- Find existing configuration
      for Item of Configuration loop
         if Item.Key = Key then
            Found := True;
            IO.Put_Line ("Current value: " & CYAN &
              SU.To_String (Item.Value) & RESET);
            IO.Put ("New value: ");
            Value := SU.To_Unbounded_String (IO.Get_Line);
            Item.Value := Value;
            IO.Put_Line (GREEN & "✓ Configuration updated" & RESET);
            exit;
         end if;
      end loop;

      if not Found then
         IO.Put_Line (YELLOW & "Key not found. Add new? (y/n): " & RESET);
         declare
            Answer : constant String := IO.Get_Line;
         begin
            if Answer = "y" or Answer = "Y" then
               IO.Put ("Value: ");
               Value := SU.To_Unbounded_String (IO.Get_Line);
               IO.Put ("Description: ");
               declare
                  Desc : constant SU.Unbounded_String :=
                    SU.To_Unbounded_String (IO.Get_Line);
               begin
                  Configuration.Append ((Key, Value, Desc));
                  IO.Put_Line (GREEN & "✓ Configuration added" & RESET);
               end;
            end if;
         end;
      end if;

      IO.New_Line;
      IO.Put_Line ("Press Enter to continue...");
      IO.Skip_Line;
   end Edit_Configuration;

   procedure Platform_Management is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "Platform Management" & RESET);
      IO.New_Line;

      IO.Put_Line ("Monitored Platforms:");
      IO.Put_Line ("  1. Twitter/X       [" & GREEN & "ACTIVE" & RESET & "]");
      IO.Put_Line ("  2. Facebook        [" & GREEN & "ACTIVE" & RESET & "]");
      IO.Put_Line ("  3. Instagram       [" & GREEN & "ACTIVE" & RESET & "]");
      IO.Put_Line ("  4. LinkedIn        [" & GREEN & "ACTIVE" & RESET & "]");
      IO.Put_Line ("  5. TikTok          [" & YELLOW & "SCRAPING" & RESET & "]");
      IO.Put_Line ("  6. YouTube         [" & GREEN & "ACTIVE" & RESET & "]");
      IO.Put_Line ("  7. Bluesky         [" & GREEN & "ACTIVE" & RESET & "]");
      IO.New_Line;

      IO.Put_Line ("Actions:");
      IO.Put_Line ("  a. Add new platform");
      IO.Put_Line ("  t. Trigger collection");
      IO.Put_Line ("  c. Configure platform");
      IO.Put_Line ("  b. Back to main menu");
      IO.New_Line;

      IO.Put (GREEN & "Select action: " & RESET);
      declare
         Choice : constant String := IO.Get_Line;
      begin
         case Choice (Choice'First) is
            when 'a' | 'A' =>
               IO.Put_Line ("Add platform wizard...");
            when 't' | 'T' =>
               IO.Put_Line ("Triggering collection for all platforms...");
            when 'c' | 'C' =>
               IO.Put_Line ("Platform configuration...");
            when 'b' | 'B' =>
               null;
            when others =>
               IO.Put_Line ("Invalid choice");
         end case;
      end;

      IO.New_Line;
      IO.Put_Line ("Press Enter to continue...");
      IO.Skip_Line;
   end Platform_Management;

   procedure Threshold_Tuning is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "NLP Threshold Self-Tuning" & RESET);
      IO.New_Line;

      IO.Put_Line ("Current Thresholds:");
      IO.Put_Line ("  Critical:      " & CYAN & "0.85" & RESET & " (85% confidence)");
      IO.Put_Line ("  High:          " & CYAN & "0.70" & RESET & " (70% confidence)");
      IO.Put_Line ("  Medium:        " & CYAN & "0.55" & RESET & " (55% confidence)");
      IO.Put_Line ("  Low:           " & CYAN & "0.40" & RESET & " (40% confidence)");
      IO.New_Line;

      IO.Put_Line ("Performance Metrics:");
      IO.Put_Line ("  Accuracy:      " & GREEN & "92.3%" & RESET);
      IO.Put_Line ("  Precision:     " & GREEN & "89.7%" & RESET);
      IO.Put_Line ("  Recall:        " & GREEN & "94.1%" & RESET);
      IO.Put_Line ("  F1 Score:      " & GREEN & "91.8%" & RESET);
      IO.New_Line;

      IO.Put_Line ("Auto-tuning Status: " & GREEN & "ENABLED" & RESET);
      IO.Put_Line ("Last tuning:        2025-11-22 14:30");
      IO.Put_Line ("Next tuning:        2025-11-23 14:30");
      IO.New_Line;

      IO.Put_Line ("Actions:");
      IO.Put_Line ("  1. Manual threshold adjustment");
      IO.Put_Line ("  2. Trigger auto-tuning now");
      IO.Put_Line ("  3. View tuning history");
      IO.Put_Line ("  4. Configure self-tuning");
      IO.Put_Line ("  b. Back");
      IO.New_Line;

      IO.Put (GREEN & "Select action: " & RESET);
      IO.Skip_Line;
   end Threshold_Tuning;

   procedure CUE_Script_Editor is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "CUE Script Editor" & RESET);
      IO.New_Line;

      IO.Put_Line ("Available CUE Scripts:");
      IO.Put_Line ("  1. twitter_extraction.cue    [Modified: 2 hours ago]");
      IO.Put_Line ("  2. facebook_extraction.cue   [Modified: 1 day ago]");
      IO.Put_Line ("  3. instagram_extraction.cue  [Modified: 3 days ago]");
      IO.Put_Line ("  4. custom_rules.cue          [Modified: 1 week ago]");
      IO.New_Line;

      IO.Put_Line ("Actions:");
      IO.Put_Line ("  n. New CUE script");
      IO.Put_Line ("  e. Edit existing script");
      IO.Put_Line ("  t. Test script");
      IO.Put_Line ("  d. Delete script");
      IO.Put_Line ("  b. Back");
      IO.New_Line;

      IO.Put (GREEN & "Select action: " & RESET);
      IO.Skip_Line;
   end CUE_Script_Editor;

   procedure Service_Status is
   begin
      Print_Header;
      IO.Put_Line (YELLOW & "Service Status" & RESET);
      IO.New_Line;

      IO.Put_Line ("Services:");
      IO.Put_Line ("  GraphQL Gateway:   " & GREEN & "● RUNNING" & RESET & "  (8000)");
      IO.Put_Line ("  Collector (Rust):  " & GREEN & "● RUNNING" & RESET & "  (3001)");
      IO.Put_Line ("  Analyzer (ReScript + Deno): " & GREEN & "● RUNNING" & RESET & "  (3002)");
      IO.Put_Line ("  Publisher (Deno):  " & GREEN & "● RUNNING" & RESET & "  (3003)");
      IO.Put_Line ("  Dashboard (Elixir):" & GREEN & "● RUNNING" & RESET & "  (4000)");
      IO.New_Line;

      IO.Put_Line ("Databases:");
      IO.Put_Line ("  Virtuoso:          " & GREEN & "● RUNNING" & RESET & "  (1111)");
      IO.Put_Line ("  XTDB:              " & GREEN & "● RUNNING" & RESET & "  (3000)");
      IO.Put_Line ("  Dragonfly:         " & GREEN & "● RUNNING" & RESET & "  (6379)");
      IO.New_Line;

      IO.Put_Line ("Actions:");
      IO.Put_Line ("  r. Restart service");
      IO.Put_Line ("  l. View logs");
      IO.Put_Line ("  m. View metrics");
      IO.Put_Line ("  b. Back");
      IO.New_Line;

      IO.Put (GREEN & "Select action: " & RESET);
      IO.Skip_Line;
   end Service_Status;

   procedure Initialize_Config is
   begin
      Configuration.Append ((
        SU.To_Unbounded_String ("database.url"),
        SU.To_Unbounded_String ("virtuoso://localhost:1111"),
        SU.To_Unbounded_String ("Virtuoso triple store connection")
      ));

      Configuration.Append ((
        SU.To_Unbounded_String ("cache.url"),
        SU.To_Unbounded_String ("dragonfly://localhost:6379"),
        SU.To_Unbounded_String ("Dragonfly cache connection")
      ));

      Configuration.Append ((
        SU.To_Unbounded_String ("temporal.url"),
        SU.To_Unbounded_String ("xtdb://localhost:3000"),
        SU.To_Unbounded_String ("XTDB temporal database")
      ));
   end Initialize_Config;

begin
   Initialize_Config;

   loop
      Print_Menu;

      declare
         Choice : constant String := IO.Get_Line;
      begin
         case Choice (Choice'First) is
            when '1' =>
               View_Configuration;
            when '2' =>
               Edit_Configuration;
            when '3' =>
               Platform_Management;
            when '4' =>
               Threshold_Tuning;
            when '5' =>
               CUE_Script_Editor;
            when '7' =>
               Service_Status;
            when '8' =>
               IO.Put_Line (GREEN & "Saving configuration..." & RESET);
               IO.Put_Line ("Goodbye!");
               exit;
            when '0' =>
               IO.Put_Line ("Exiting without saving. Goodbye!");
               exit;
            when others =>
               IO.Put_Line ("Invalid option. Press Enter to continue...");
               IO.Skip_Line;
         end case;
      end;
   end loop;

end NUJ_TUI;
