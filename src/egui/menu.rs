use crate::egui::{GuiState, GuiWindow, GuiEvent, GuiOption};


impl GuiState {

    pub fn draw_menu(&mut self, ui: &mut egui::Ui) {

        egui::menu::bar(ui, |ui| {

            ui.menu_button("Emulator", |ui| {
                if ui.button("Performance...").clicked() {
                    *self.window_flag(GuiWindow::PerfViewer) = true;
                    ui.close_menu();
                }
                if ui.button("About...").clicked() {
                    *self.window_flag(GuiWindow::About) = true;
                    ui.close_menu();
                }                    
            });
            ui.menu_button("Machine", |ui| {
                if ui.button("⚡ Power on").clicked() {
                    
                    ui.close_menu();
                } 
                if ui.checkbox(&mut self.get_option_mut(GuiOption::TurboButton), "Turbo Button").clicked() {

                    let new_opt = self.get_option(GuiOption::TurboButton).unwrap();

                    self.event_queue.push_back(
                        GuiEvent::OptionChanged(
                            GuiOption::TurboButton, 
                            new_opt 
                        )
                    );
                    ui.close_menu();
                }
                if ui.button("⏸ Pause").clicked() {
                    
                    ui.close_menu();
                }
                if ui.button("⟲ Reboot").clicked() {
                    
                    ui.close_menu();
                }                    
                if ui.button("🔌 Power off").clicked() {
                    
                    ui.close_menu();
                }                   
            });
            ui.menu_button("Media", |ui| {

                ui.set_min_size(egui::vec2(200.0, 0.0));
                //ui.style_mut().spacing.item_spacing = egui::Vec2{ x: 6.0, y:6.0 };

                ui.menu_button("💾 Load Floppy in Drive A:...", |ui| {
                    for name in &self.floppy_names {
                        if ui.button(name.to_str().unwrap()).clicked() {
                            
                            log::debug!("Selected floppy filename: {:?}", name);
                            
                            self.new_floppy_name0 = Some(name.clone());
                            self.event_queue.push_back(GuiEvent::LoadFloppy(0, name.clone()));
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("💾 Load Floppy in Drive B:...", |ui| {
                    for name in &self.floppy_names {
                        if ui.button(name.to_str().unwrap()).clicked() {
                            
                            log::debug!("Selected floppy filename: {:?}", name);
                            
                            self.new_floppy_name1 = Some(name.clone());
                            self.event_queue.push_back(GuiEvent::LoadFloppy(1, name.clone()));
                            ui.close_menu();
                        }
                    }
                });      
                
                if ui.button("⏏ Eject Floppy in Drive A:").clicked() {
                    self.event_queue.push_back(GuiEvent::EjectFloppy(0));
                    ui.close_menu();
                };       
                
                if ui.button("⏏ Eject Floppy in Drive B:").clicked() {
                    self.event_queue.push_back(GuiEvent::EjectFloppy(1));
                    ui.close_menu();
                };                              

                ui.menu_button("🖴 Load VHD in Drive 0:...", |ui| {
                    for name in &self.vhd_names {

                        if ui.radio_value(&mut self.vhd_name0, name.clone(), name.to_str().unwrap()).clicked() {

                            log::debug!("Selected VHD filename: {:?}", name);
                            self.new_vhd_name0 = Some(name.clone());
                            ui.close_menu();
                        }
                    }
                });                               

                if ui.button("🖹 Create new VHD...").clicked() {
                    *self.window_flag(GuiWindow::VHDCreator) = true;
                    ui.close_menu();
                };
                
            });
            ui.menu_button("Debug", |ui| {
                ui.menu_button("Dump Memory", |ui| {
                    if ui.button("Video Memory").clicked() {
                        self.event_queue.push_back(GuiEvent::DumpVRAM);
                        ui.close_menu();
                    }
                    if ui.button("Code Segment").clicked() {
                        self.event_queue.push_back(GuiEvent::DumpCS);
                        ui.close_menu();
                    }
                    if ui.button("All Memory").clicked() {
                        self.event_queue.push_back(GuiEvent::DumpAllMem);
                        ui.close_menu();
                    }                    
                });
                if ui.button("CPU Control...").clicked() {
                    *self.window_flag(GuiWindow::CpuControl) = true;
                    ui.close_menu();
                }
                if ui.button("CPU State...").clicked() {
                    *self.window_flag(GuiWindow::CpuStateViewer) = true;
                    ui.close_menu();
                }
                ui.menu_button("CPU Debug Options", |ui| {
                    if ui.checkbox(&mut self.get_option_mut(GuiOption::CpuEnableWaitStates), "Enable Wait States").clicked() {

                        let new_opt = self.get_option(GuiOption::CpuEnableWaitStates).unwrap();
    
                        self.event_queue.push_back(
                            GuiEvent::OptionChanged(
                                GuiOption::CpuEnableWaitStates, 
                                new_opt 
                            )
                        );
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.get_option_mut(GuiOption::CpuInstructionHistory), "Instruction History").clicked() {

                        let new_opt = self.get_option(GuiOption::CpuInstructionHistory).unwrap();
    
                        self.event_queue.push_back(
                            GuiEvent::OptionChanged(
                                GuiOption::CpuInstructionHistory, 
                                new_opt 
                            )
                        );
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.get_option_mut(GuiOption::CpuTraceLoggingEnabled), "Trace Logging Enabled").clicked() {

                        let new_opt = self.get_option(GuiOption::CpuTraceLoggingEnabled).unwrap();
    
                        self.event_queue.push_back(
                            GuiEvent::OptionChanged(
                                GuiOption::CpuTraceLoggingEnabled, 
                                new_opt 
                            )
                        );
                        ui.close_menu();
                    }   
                    if ui.button("Delays...").clicked() {
                        *self.window_flag(GuiWindow::DelayAdjust) = true;
                        ui.close_menu();
                    }

                });
                if ui.button("Memory...").clicked() {
                    *self.window_flag(GuiWindow::MemoryViewer) = true;
                    ui.close_menu();
                }
                if ui.button("Instruction History...").clicked() {
                    *self.window_flag(GuiWindow::HistoryViewer) = true;
                    ui.close_menu();
                }
                if ui.button("Instruction Cycle Trace...").clicked() {
                    *self.window_flag(GuiWindow::CycleTraceViewer) = true;
                    ui.close_menu();
                }                
                if ui.button("Call Stack...").clicked() {
                    *self.window_flag(GuiWindow::CallStack) = true;
                    ui.close_menu();
                }                    
                if ui.button("Disassembly...").clicked() {
                    *self.window_flag(GuiWindow::DiassemblyViewer) = true;
                    ui.close_menu();
                }
                if ui.button("IVR...").clicked() {
                    *self.window_flag(GuiWindow::IvrViewer) = true;
                    ui.close_menu();
                }                
                if ui.button("PIC...").clicked() {
                    *self.window_flag(GuiWindow::PicViewer) = true;
                    ui.close_menu();
                }    
                if ui.button("PIT...").clicked() {
                    *self.window_flag(GuiWindow::PitViewer) = true;
                    ui.close_menu();
                }
                if ui.button("PPI...").clicked() {
                    *self.window_flag(GuiWindow::PpiViewer) = true;
                    ui.close_menu();
                }
                if ui.button("DMA...").clicked() {
                    *self.window_flag(GuiWindow::DmaViewer) = true;
                    ui.close_menu();
                }
                if ui.button("Video Card...").clicked() {
                    *self.window_flag(GuiWindow::VideoCardViewer) = true;
                    ui.close_menu();
                }
                if ui.button("Flush Trace Logs").clicked() {
                    self.event_queue.push_back(GuiEvent::FlushLogs);
                    ui.close_menu();
                }
            });
            ui.menu_button("Options", |ui| {


                ui.menu_button("Display", |ui| {
                    if ui.checkbox(&mut self.get_option_mut(GuiOption::CorrectAspect), "Correct Aspect Ratio").clicked() {

                        let new_opt = self.get_option(GuiOption::CorrectAspect).unwrap();
    
                        self.event_queue.push_back(
                            GuiEvent::OptionChanged(
                                GuiOption::CorrectAspect, 
                                new_opt 
                            )
                        );
                        ui.close_menu();
                    }
                    if ui.checkbox(&mut self.composite, "Composite Monitor").clicked() {
                        ui.close_menu();
                    }

                    if ui.button("Composite Adjustments...").clicked() {
                        *self.window_flag(GuiWindow::CompositeAdjust) = true;
                        ui.close_menu();
                    }             
                });                

                ui.menu_button("Attach COM2: ...", |ui| {
                    for port in &self.serial_ports {

                        if ui.radio_value(&mut self.serial_port_name, port.port_name.clone(), port.port_name.clone()).clicked() {

                            self.event_queue.push_back(GuiEvent::BridgeSerialPort(self.serial_port_name.clone()));
                            ui.close_menu();
                        }
                    }
                });                                
            });
        });

    }
}