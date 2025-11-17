import { useEffect, useState } from "react";
import { MealTemplate, SlotType } from "../../lib/types";
import { MealSelectionModal } from "./MealSelectionModal";
import { OptionSelectionModal } from "./OptionSelectionModal";

interface MealWizardModalProps {
  isOpen: boolean;
  onClose: () => void;
  slotType: SlotType;
  slotName: string;
  date: string;
  onSuccess: () => void;
}

/**
 * MealWizardModal - Two-step wizard for adding a meal to a slot
 * Step 1: Select template (MealSelectionModal)
 * Step 2: Select option and configure details (OptionSelectionModal)
 * 
 * This coordinates the two-step flow with seamless transitions and a back button.
 */
export function MealWizardModal({
  isOpen,
  onClose,
  slotType,
  slotName,
  date,
  onSuccess,
}: MealWizardModalProps) {
  const [step, setStep] = useState<1 | 2>(1);
  const [selectedTemplate, setSelectedTemplate] = useState<MealTemplate | null>(
    null
  );

  // Reset state when wizard closes
  useEffect(() => {
    if (!isOpen) {
      setStep(1);
      setSelectedTemplate(null);
    }
  }, [isOpen]);

  // Handle template selection - move to step 2
  const handleTemplateSelect = (template: MealTemplate) => {
    setSelectedTemplate(template);
    setStep(2);
  };

  // Handle going back to step 1
  const handleBack = () => {
    setStep(1);
    setSelectedTemplate(null);
  };

  // Handle successful meal addition
  const handleSuccess = () => {
    onSuccess();
    // State reset handled by useEffect when modal closes
  };

  // Handle closing the wizard
  const handleClose = () => {
    onClose();
    // State reset handled by useEffect
  };

  return (
    <>
      {/* Step 1: Template Selection */}
      {step === 1 && isOpen && (
        <MealSelectionModal
          key="step-1"
          isOpen={true}
          onClose={handleClose}
          slotType={slotType}
          slotName={slotName}
          onSelectTemplate={handleTemplateSelect}
        />
      )}

      {/* Step 2: Option Selection */}
      {step === 2 && selectedTemplate && isOpen && (
        <OptionSelectionModal
          key="step-2"
          isOpen={true}
          onClose={handleClose}
          template={selectedTemplate}
          slotType={slotType}
          slotName={slotName}
          date={date}
          onSuccess={handleSuccess}
          onBack={handleBack}
        />
      )}
    </>
  );
}
