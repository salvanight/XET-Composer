'use client'; // Required for event handlers and state

import { useState, FormEvent } from 'react';

interface VestingFormData {
  tokenAddress: string;
  beneficiaryAddress: string;
  startTime: string; // Keep as string from datetime-local input
  cliffDurationSeconds: string; // In seconds
  totalVestingSeconds: string; // In seconds
  initialOwnerAddress: string;
}

export default function VestingPage() {
  const [formData, setFormData] = useState<VestingFormData>({
    tokenAddress: '0xTokenAddressHere', // Default placeholder
    beneficiaryAddress: '0xBeneficiaryAddressHere', // Default placeholder
    startTime: '',
    cliffDurationSeconds: '2592000', // 30 days in seconds
    totalVestingSeconds: '31536000', // 1 year in seconds
    initialOwnerAddress: '0xInitialOwnerAddressHere', // Default placeholder
  });
  const [submissionResult, setSubmissionResult] = useState<string | null>(null);
  const [deployedContractInfo, setDeployedContractInfo] = useState<any | null>(null); // Added state

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSubmissionResult('Submitting...');
    setDeployedContractInfo(null); // Clear previous contract info
    console.log('Form Data:', formData);

    // Convert times to Unix timestamps if needed by backend, or send as ISO strings
    // For start_time, it's often easier to send as Unix timestamp.
    // Cliff and duration are already in seconds.
    const payload = {
      contract: "TokenVesting",
      params: {
        token_address: formData.tokenAddress,
        beneficiary: formData.beneficiaryAddress,
        // Convert startTime to Unix timestamp (seconds) before sending
        start_time: formData.startTime ? Math.floor(new Date(formData.startTime).getTime() / 1000) : Math.floor(Date.now() / 1000),
        cliff_duration: parseInt(formData.cliffDurationSeconds, 10),
        duration: parseInt(formData.totalVestingSeconds, 10),
        initial_owner: formData.initialOwnerAddress,
      }
    };

    console.log('Sending payload to backend:', JSON.stringify(payload, null, 2));

    try {
      // Placeholder for API call. This will be fully implemented when backend is ready.
      const response = await fetch('/api/deploy', { // Assuming Next.js API route or direct backend
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        const errorData = await response.text();
        throw new Error(`HTTP error ${response.status}: ${errorData}`);
      }

      const result = await response.json();
      console.log('Deployment Result:', result);
      if (result.success) {
        setDeployedContractInfo(result);
        setSubmissionResult(`Deployment successful: ${result.message || 'See details below.'}`);
      } else {
        setSubmissionResult(`Deployment failed: ${result.message || 'Unknown error.'}`);
        setDeployedContractInfo(null); // Clear if backend indicates failure
      }
    } catch (error: any) {
      console.error('Deployment Error:', error);
      setSubmissionResult(`Error: ${error.message}`);
      setDeployedContractInfo(null); // Clear contract info on error
    }
  };

  // Get current datetime for min attribute of datetime-local input
  const nowForInput = new Date().toISOString().slice(0, 16);

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24 bg-gray-900 text-white">
      <div className="bg-gray-800 p-8 rounded-lg shadow-xl w-full max-w-2xl">
        <h1 className="text-3xl font-bold mb-6 text-center text-purple-400">Deploy Token Vesting Contract</h1>
        
        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label htmlFor="tokenAddress" className="block text-sm font-medium text-gray-300 mb-1">Token Address (ERC20)</label>
            <input
              type="text"
              name="tokenAddress"
              id="tokenAddress"
              value={formData.tokenAddress}
              onChange={handleChange}
              required
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <div>
            <label htmlFor="beneficiaryAddress" className="block text-sm font-medium text-gray-300 mb-1">Beneficiary Address</label>
            <input
              type="text"
              name="beneficiaryAddress"
              id="beneficiaryAddress"
              value={formData.beneficiaryAddress}
              onChange={handleChange}
              required
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <div>
            <label htmlFor="startTime" className="block text-sm font-medium text-gray-300 mb-1">Start Time (UTC)</label>
            <input
              type="datetime-local"
              name="startTime"
              id="startTime"
              value={formData.startTime}
              onChange={handleChange}
              required
              min={nowForInput} // Prevent selecting past dates
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <div>
            <label htmlFor="cliffDurationSeconds" className="block text-sm font-medium text-gray-300 mb-1">Cliff Duration (seconds)</label>
            <input
              type="number"
              name="cliffDurationSeconds"
              id="cliffDurationSeconds"
              value={formData.cliffDurationSeconds}
              onChange={handleChange}
              required
              min="0"
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <div>
            <label htmlFor="totalVestingSeconds" className="block text-sm font-medium text-gray-300 mb-1">Total Vesting Duration (seconds)</label>
            <input
              type="number"
              name="totalVestingSeconds"
              id="totalVestingSeconds"
              value={formData.totalVestingSeconds}
              onChange={handleChange}
              required
              min="0"
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <div>
            <label htmlFor="initialOwnerAddress" className="block text-sm font-medium text-gray-300 mb-1">Initial Owner Address</label>
            <input
              type="text"
              name="initialOwnerAddress"
              id="initialOwnerAddress"
              value={formData.initialOwnerAddress}
              onChange={handleChange}
              required
              className="w-full bg-gray-700 border border-gray-600 text-white rounded-md p-2 focus:ring-purple-500 focus:border-purple-500"
            />
          </div>

          <button 
            type="submit"
            className="w-full bg-purple-600 hover:bg-purple-700 text-white font-semibold py-3 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 focus:ring-offset-gray-800 transition duration-150 ease-in-out"
          >
            Deploy Vesting Contract
          </button>
        </form>

        {submissionResult && (
          <div className="mt-6 p-4 bg-gray-700 rounded-md">
            <h3 className="text-lg font-semibold mb-2">Submission Status:</h3>
            <pre className="text-sm text-gray-300 whitespace-pre-wrap break-all">{submissionResult}</pre>
            {deployedContractInfo && deployedContractInfo.success && (
              <div className="mt-4">
                <h4 className="text-md font-semibold text-purple-300">Contract Details:</h4>
                <p className="text-sm text-gray-300">
                  Address: <a 
                    href={`https://sepolia.etherscan.io/address/${deployedContractInfo.contract_address}`} 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="text-blue-400 hover:text-blue-300 underline"
                  >
                    {deployedContractInfo.contract_address}
                  </a>
                </p>
                <p className="text-sm text-gray-300 mt-1">Message: {deployedContractInfo.message}</p>
                <div className="mt-2">
                  <p className="text-sm font-medium text-gray-300">ABI:</p>
                  <pre className="text-xs bg-gray-600 p-2 rounded-md text-gray-200 whitespace-pre-wrap break-all">
                    {JSON.stringify(deployedContractInfo.abi, null, 2)}
                  </pre>
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </main>
  );
}
